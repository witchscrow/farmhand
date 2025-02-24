use super::config::Config;
use crate::{
    db::connect_to_database,
    nats::create_nats_client,
    storage::s3::create_s3_client,
    workers::{events::Stream, Queue},
};
use sqlx::PgPool;

/// Shared state available to the API
pub struct AppState {
    pub db: PgPool,
    pub job_queue: Queue,
    pub event_stream: Stream,
    pub config: Config,
    pub s3_client: aws_sdk_s3::Client,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize a connection to the database
        let db = connect_to_database()
            .await
            .expect("Could not connect to database");

        // Create the S3 Client
        let s3_client = create_s3_client().await;

        // Create a NATS client
        let nats_client = create_nats_client().await?;

        // Connect to the job queue
        let job_queue = Queue::connect(nats_client.clone())
            .await
            .expect("Failed to create worker queue");

        // Connect to the event stream
        let event_stream = Stream::connect(nats_client.clone())
            .await
            .expect("Failed to connect to event stream");

        Ok(Self {
            config,
            db,
            job_queue,
            event_stream,
            s3_client,
        })
    }
}
