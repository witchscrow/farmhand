pub mod stream;
pub use stream::Stream;

use crate::twitch::{ChatMessagePayload, StreamStatusPayload};
pub use stream::EVENT_STREAM;

pub const MESSAGE_PREFIX: &str = "farmhand";
pub const EVENT_PREFIX: &str = "events";
pub const JOB_PREFIX: &str = "jobs";
pub const JOB_STREAM: &str = "FARMHAND_JOBS";

/// Represents events we send and receive from NATS
/// Primarily used to get the appropriate subject name for an event
pub enum Event {
    ChatMessage(ChatMessagePayload),
    StreamStatus(StreamStatusPayload),
}

impl Event {
    pub fn get_subject(&self) -> String {
        match self {
            // farmhand.events.twitch.{broadcaster_name}.chat_message
            Event::ChatMessage(payload) => format!(
                "{}.{}.twitch.events.{}.chat_message",
                MESSAGE_PREFIX, EVENT_PREFIX, payload.broadcaster_user_name
            ),
            // farmhand.events.twitch.{broadcaster_name}.stream_status
            Event::StreamStatus(payload) => {
                let status = if payload.is_online() {
                    "online"
                } else {
                    "offline"
                };
                format!(
                    "{}.{}.twitch.events.{}.stream_{}",
                    MESSAGE_PREFIX, EVENT_PREFIX, payload.event.broadcaster_user_name, status
                )
            }
        }
    }
}

impl From<ChatMessagePayload> for Event {
    fn from(payload: ChatMessagePayload) -> Self {
        Event::ChatMessage(payload)
    }
}

impl From<StreamStatusPayload> for Event {
    fn from(payload: StreamStatusPayload) -> Self {
        Event::StreamStatus(payload)
    }
}
