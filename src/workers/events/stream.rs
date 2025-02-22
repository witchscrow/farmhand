use async_nats::{
    jetstream::{
        self,
        consumer::{pull::Config, Consumer},
        Context,
    },
    Client,
};

use crate::{error::StreamError, workers::EVENT_STREAM};

#[allow(dead_code)]
/// TODO: Remove dead code annotation after implementing
pub struct Stream {
    name: String,
    jetstream: Context,
}

impl Stream {
    /// Connects to an existing queue
    pub async fn connect(nats_client: Client) -> Result<Self, StreamError> {
        let jetstream = Self::create_jetstream(nats_client);
        jetstream
            .get_stream(EVENT_STREAM)
            .await
            .map_err(|e| StreamError::InvalidConnection(e.to_string()))?;
        Ok(Stream {
            name: EVENT_STREAM.to_string(),
            jetstream,
        })
    }
    /// Creates a new queue
    pub async fn new(
        name: String,
        description: Option<String>,
        subjects: Vec<String>,
        nats_client: Client,
    ) -> Result<Self, StreamError> {
        let jetstream = Self::create_jetstream(nats_client);
        jetstream
            .create_stream(jetstream::stream::Config {
                name: name.clone(),
                subjects,
                description,
                max_bytes: 1024 * 1024 * 1024, // 1GB
                ..Default::default()
            })
            .await
            .map_err(|e| StreamError::InvalidConnection(e.to_string()))?;
        Ok(Stream { name, jetstream })
    }
    /// Deletes the stream
    pub async fn delete(nats_client: Client) -> Result<(), StreamError> {
        let jetstream = Self::create_jetstream(nats_client);

        // Check if stream exists first
        if jetstream.get_stream(EVENT_STREAM).await.is_ok() {
            jetstream
                .delete_stream(EVENT_STREAM)
                .await
                .map_err(|e| StreamError::InvalidConnection(e.to_string()))?;
        } else {
            tracing::warn!("Stream {} does not exist", EVENT_STREAM);
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
    ) -> Result<Consumer<Config>, StreamError> {
        let config = jetstream::consumer::pull::Config {
            durable_name: name,
            filter_subject: filter,
            max_deliver: 3,
            ..Default::default()
        };
        self.jetstream
            .create_consumer_on_stream(config, self.name.to_string())
            .await
            .map_err(|e| StreamError::InvalidConnection(e.to_string()))
    }
    /// Publishes a message to the queue
    pub async fn publish(&self, subject: String, message: String) -> Result<(), StreamError> {
        tracing::debug!("Publishing message to subject {}", subject);
        self.jetstream
            .publish(subject, message.into())
            .await
            .map_err(|e| StreamError::InvalidConnection(e.to_string()))?;

        Ok(())
    }
}
