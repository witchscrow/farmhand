use async_nats::{
    jetstream::{
        self,
        consumer::{pull::Config, Consumer},
        stream::RetentionPolicy,
        Context,
    },
    Client,
};

use crate::{error::QueueError, workers::JOB_STREAM};

#[allow(dead_code)]
/// TODO: Remove dead code annotation after implementing
pub struct Queue {
    name: String,
    jetstream: Context,
}

impl Queue {
    /// Connects to an existing queue
    pub async fn connect(nats_client: Client) -> Result<Self, QueueError> {
        let jetstream = Self::create_jetstream(nats_client);
        jetstream
            .get_stream(JOB_STREAM)
            .await
            .map_err(|e| QueueError::InvalidConnection(e.to_string()))?;
        Ok(Queue {
            name: JOB_STREAM.to_string(),
            jetstream,
        })
    }
    /// Creates a new queue
    pub async fn new(
        name: String,
        description: Option<String>,
        subjects: Vec<String>,
        nats_client: Client,
    ) -> Result<Self, QueueError> {
        let jetstream = Self::create_jetstream(nats_client);
        jetstream
            .create_stream(jetstream::stream::Config {
                name: name.clone(),
                subjects,
                description,
                retention: RetentionPolicy::WorkQueue,
                ..Default::default()
            })
            .await
            .map_err(|e| QueueError::InvalidConnection(e.to_string()))?;
        Ok(Queue { name, jetstream })
    }
    /// Deletes the queue
    pub async fn delete(nats_client: Client) -> Result<(), QueueError> {
        let jetstream = Self::create_jetstream(nats_client);

        // Check if stream exists first
        if jetstream.get_stream(JOB_STREAM).await.is_ok() {
            jetstream
                .delete_stream(JOB_STREAM)
                .await
                .map_err(|e| QueueError::InvalidConnection(e.to_string()))?;
        } else {
            tracing::warn!("Stream {} does not exist", JOB_STREAM);
        }
        Ok(())
    }
    /// Creates a new jetstream context
    fn create_jetstream(nats_client: Client) -> Context {
        jetstream::new(nats_client)
    }
    /// Creates a new consumer
    pub async fn create_consumer(
        &self,
        name: Option<String>,
        filter: String,
    ) -> Result<Consumer<Config>, QueueError> {
        let config = jetstream::consumer::pull::Config {
            durable_name: name,
            filter_subject: filter,
            max_deliver: 3,
            ..Default::default()
        };
        self.jetstream
            .create_consumer_on_stream(config, self.name.to_string())
            .await
            .map_err(|e| QueueError::InvalidConnection(e.to_string()))
    }
    /// Publishes a message to the queue
    pub async fn publish(&self, subject: String, message: String) -> Result<(), QueueError> {
        tracing::debug!("Publishing message to subject {}", subject);
        self.jetstream
            .publish(subject, message.into())
            .await
            .map_err(|e| QueueError::InvalidConnection(e.to_string()))?;

        Ok(())
    }
}
