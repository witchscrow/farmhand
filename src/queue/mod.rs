pub mod hls_stream;
pub mod queue;

use anyhow::Result;
use async_nats::Message;
use hls_stream::HlsStreamRunner;
pub use queue::Queue;
use serde::de::DeserializeOwned;

/// Creates the appropriate runner based on the subject, then runs it
pub async fn process_message(message: &Message) -> Result<()> {
    let subject = message.subject.as_str();
    let runner = RunnerType::from_subject(subject)?;
    runner.run(message).await
}

pub(crate) trait Runner: Send + Sync + 'static {
    type Payload: DeserializeOwned;

    /// Parses a payload from NATS into the expected format
    async fn parse_payload(&self, message: &Message) -> Result<Self::Payload> {
        let payload = message.payload.clone();
        let payload = serde_json::from_slice::<Self::Payload>(&payload)?;

        Ok(payload)
    }
    /// Parses the payload and runs the job
    async fn run(&self, message: &Message) -> Result<()> {
        let payload = self.parse_payload(message).await?;
        self.process_job(payload).await
    }
    /// Processes the job
    async fn process_job(&self, payload: Self::Payload) -> Result<()>;
}

/// Represents the different types of runners that can be used in the application
pub enum RunnerType {
    TransformVideo(HlsStreamRunner),
}

impl RunnerType {
    /// Creates a new runner from a subject
    pub fn from_subject(subject: &str) -> Result<Self> {
        tracing::debug!("Creating runner for subject: {}", subject);
        match subject {
            "farmhand.jobs.video_to_stream" => Ok(RunnerType::TransformVideo(HlsStreamRunner)),
            _ => Err(anyhow::anyhow!("{} has no runner associated", subject)),
        }
    }
    /// Method to run the appropriate runner
    pub async fn run(&self, message: &Message) -> Result<()> {
        match self {
            RunnerType::TransformVideo(runner) => runner.run(message).await,
        }
    }
}
