use super::runner::chat::ChatMessagePayload;

/// Represents events we send and receive from NATS
/// Primarily used to get the appropriate subject name for an event
pub enum Event {
    ChatMessage(ChatMessagePayload),
}

const EVENT_PREFIX: &str = "farmhand";

impl Event {
    pub fn get_subject(&self) -> String {
        match self {
            // twitch.events.{broadcaster_name}.chat_message
            Event::ChatMessage(payload) => format!(
                "{}.twitch.events.{}.chat_message",
                EVENT_PREFIX, payload.broadcaster_user_name
            ),
        }
    }
}

impl From<ChatMessagePayload> for Event {
    fn from(payload: ChatMessagePayload) -> Self {
        Event::ChatMessage(payload)
    }
}
