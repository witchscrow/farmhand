use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::db::accounts::Account;

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamStatusPayload {
    pub subscription: super::subscription::Subscription,
    pub event: StreamEvent,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamEvent {
    #[serde(default)]
    pub id: Option<String>,
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    #[serde(default, rename = "type")]
    pub stream_type: Option<String>,
    #[serde(default)]
    pub started_at: Option<String>,
}

impl StreamStatusPayload {
    /// Check if this is an online event
    pub fn is_online(&self) -> bool {
        self.subscription.event_type == "stream.online"
    }

    /// Check if this is an offline event
    pub fn is_offline(&self) -> bool {
        self.subscription.event_type == "stream.offline"
    }

    /// Get the started timestamp as DateTime if available
    pub fn started_at(&self) -> Option<DateTime<Utc>> {
        self.event
            .started_at
            .as_ref()
            .and_then(|ts| DateTime::parse_from_rfc3339(ts).ok())
            .map(|dt| dt.with_timezone(&Utc))
    }

    /// Find the associated user account based on the broadcaster ID
    pub async fn find_broadcaster_account(&self, pool: &PgPool) -> Result<Account, sqlx::Error> {
        Account::find_by_provider("twitch", &self.event.broadcaster_user_id, pool).await
    }
}
