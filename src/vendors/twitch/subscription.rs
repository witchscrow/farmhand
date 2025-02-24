use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscription {
    pub id: String,
    pub status: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub version: String,
    pub cost: i32,
    pub condition: serde_json::Value,
    pub transport: Transport,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transport {
    pub method: String,
    pub callback: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Notification {
    pub subscription: Subscription,
    pub event: Option<serde_json::Value>,
    pub challenge: Option<String>,
}
