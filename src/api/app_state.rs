use super::config::Config;
use crate::{db::connect_to_database, storage::s3::create_s3_client, workers};
use sqlx::PgPool;

/// Shared state available to the API
pub struct AppState {
    pub db: PgPool,
    pub job_queue: workers::Queue,
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

        // Connect to the job queue
        let nats_client = workers::create_nats_client().await?;
        let jq_name = "FARMHAND_JOBS".to_string();
        let job_queue = workers::Queue::connect(jq_name, nats_client)
            .await
            .expect("Failed to create worker queue");

        Ok(Self {
            config,
            db,
            job_queue,
            s3_client,
        })
    }
}
