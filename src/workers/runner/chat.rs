use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::Runner;

#[derive(Deserialize, Serialize)]
pub struct ChatMessagePayload {
    pub message: String,
    pub chatter: User,
    pub broadcaster: User,
}

impl ChatMessagePayload {
    pub fn new(message: String, chatter: User, broadcaster: User) -> Self {
        Self {
            message,
            chatter,
            broadcaster,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub id: String,
}

impl User {
    pub fn new(username: String, id: String) -> Self {
        Self { username, id }
    }
}

/// A runner for processing chat messages
pub struct ChatMessageRunner;

impl Runner for ChatMessageRunner {
    type Payload = ChatMessagePayload;

    async fn process_job(&self, payload: Self::Payload) -> Result<()> {
        tracing::debug!(
            "Processing job with runner ChatMessageRunner for broadcaster: {broadcaster}",
            broadcaster = payload.broadcaster.username
        );
        Ok(())
    }
}
