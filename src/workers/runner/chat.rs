use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::Runner;

#[derive(Deserialize, Serialize)]
pub struct ChatMessagePayload {
    pub message: Message,
    #[serde(rename = "chatter_user_id")]
    pub chatter_user_id: String,
    #[serde(rename = "chatter_user_login")]
    pub chatter_user_login: String,
    #[serde(rename = "chatter_user_name")]
    pub chatter_user_name: String,
    #[serde(rename = "broadcaster_user_id")]
    pub broadcaster_user_id: String,
    #[serde(rename = "broadcaster_user_login")]
    pub broadcaster_user_login: String,
    #[serde(rename = "broadcaster_user_name")]
    pub broadcaster_user_name: String,
    pub message_id: String,
    pub message_type: String,
    pub color: Option<String>,
    pub badges: Option<Vec<Badge>>,
    pub reply: Option<serde_json::Value>,
    #[serde(rename = "channel_points_custom_reward_id")]
    pub channel_points_custom_reward_id: Option<String>,
    #[serde(rename = "channel_points_animation_id")]
    pub channel_points_animation_id: Option<String>,
    pub cheer: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub text: String,
    pub fragments: Vec<MessageFragment>,
}

#[derive(Deserialize, Serialize)]
pub struct MessageFragment {
    pub text: String,
    #[serde(rename = "type")]
    pub fragment_type: String,
    pub emote: Option<serde_json::Value>,
    pub mention: Option<serde_json::Value>,
    pub cheermote: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Badge {
    pub id: String,
    pub info: String,
    #[serde(rename = "set_id")]
    pub set_id: String,
}

impl ChatMessagePayload {
    pub fn new(
        message: Message,
        chatter_user_id: String,
        chatter_user_login: String,
        chatter_user_name: String,
        broadcaster_user_id: String,
        broadcaster_user_login: String,
        broadcaster_user_name: String,
        message_id: String,
        message_type: String,
    ) -> Self {
        Self {
            message,
            chatter_user_id,
            chatter_user_login,
            chatter_user_name,
            broadcaster_user_id,
            broadcaster_user_login,
            broadcaster_user_name,
            message_id,
            message_type,
            color: None,
            badges: None,
            reply: None,
            channel_points_custom_reward_id: None,
            channel_points_animation_id: None,
            cheer: None,
        }
    }
}

/// A runner for processing chat messages
pub struct ChatMessageRunner;

impl Runner for ChatMessageRunner {
    type Payload = ChatMessagePayload;

    async fn process_job(&self, payload: Self::Payload) -> Result<()> {
        tracing::debug!(
            "Processing job with runner ChatMessageRunner for broadcaster: {broadcaster}",
            broadcaster = payload.broadcaster_user_name
        );
        Ok(())
    }
}
