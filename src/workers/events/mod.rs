use chat::ChatMessagePayload;

pub mod chat;
pub mod stream;
pub use stream::Stream;

/// Represents events we send and receive from NATS
/// Primarily used to get the appropriate subject name for an event
pub enum Event {
    ChatMessage(ChatMessagePayload),
}

pub const MESSAGE_PREFIX: &str = "farmhand";
pub const EVENT_PREFIX: &str = "events";
pub const JOB_PREFIX: &str = "jobs";
pub const EVENT_STREAM: &str = "FARMHAND_EVENTS";
pub const JOB_STREAM: &str = "FARMHAND_JOBS";

impl Event {
    pub fn get_subject(&self) -> String {
        match self {
            // farmhand.events.twitch.{broadcaster_name}.chat_message
            Event::ChatMessage(payload) => format!(
                "{}.{}.twitch.events.{}.chat_message",
                MESSAGE_PREFIX, EVENT_PREFIX, payload.broadcaster_user_name
            ),
        }
    }
}

impl From<ChatMessagePayload> for Event {
    fn from(payload: ChatMessagePayload) -> Self {
        Event::ChatMessage(payload)
    }
}
