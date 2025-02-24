pub mod stream;
use serde::{Deserialize, Serialize};
pub use stream::Stream;
use uuid::Uuid;

use crate::twitch::{ChatMessagePayload, StreamStatusPayload};
pub use stream::EVENT_STREAM;

pub const MESSAGE_PREFIX: &str = "farmhand";
pub const EVENT_PREFIX: &str = "events";
pub const JOB_PREFIX: &str = "jobs";
pub const JOB_STREAM: &str = "FARMHAND_JOBS";

#[derive(Serialize, Deserialize)]
pub struct Event {
    payload: EventPayload,
    stream_db_id: Option<Uuid>,
}

/// Represents event types we send and receive from NATS
#[derive(Serialize, Deserialize)]
pub enum EventPayload {
    ChatMessage(ChatMessagePayload),
    StreamStatus(StreamStatusPayload),
}

impl Event {
    pub fn get_subject(&self) -> String {
        let raw_subject = match &self.payload {
            // farmhand.events.twitch.{broadcaster_name}.chat_message
            EventPayload::ChatMessage(payload) => format!(
                "{}.{}.twitch.events.{}.chat_message",
                MESSAGE_PREFIX, EVENT_PREFIX, payload.broadcaster_user_name
            ),
            // farmhand.events.twitch.{broadcaster_name}.stream_status
            EventPayload::StreamStatus(payload) => {
                let status = if payload.started_at.is_some() {
                    "online"
                } else {
                    "offline"
                };
                format!(
                    "{}.{}.twitch.events.{}.stream_{}",
                    MESSAGE_PREFIX, EVENT_PREFIX, payload.broadcaster_user_name, status
                )
            }
        };
        // Make sure the subject is lowercase
        raw_subject.to_lowercase()
    }
    pub fn set_stream_db_id(mut self, stream_db_id: Uuid) -> Self {
        self.stream_db_id = Some(stream_db_id);
        self
    }
}

impl From<ChatMessagePayload> for Event {
    fn from(payload: ChatMessagePayload) -> Self {
        Event {
            payload: EventPayload::ChatMessage(payload),
            stream_db_id: None,
        }
    }
}

impl From<StreamStatusPayload> for Event {
    fn from(payload: StreamStatusPayload) -> Self {
        Event {
            payload: EventPayload::StreamStatus(payload),
            stream_db_id: None,
        }
    }
}
