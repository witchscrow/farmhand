use std::sync::Arc;

use queue::{PostgresQueue, Queue};
use sqlx::PgPool;

use crate::{config::Config, s3::create_s3_client};

/// Shared state available to the API
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
    pub queue: Arc<dyn Queue>,
    pub s3_client: aws_sdk_s3::Client,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize a connection to the database
        let db = db::connect_to_database()
            .await
            .expect("Could not connect to database");
        // Initialize the queue
        let queue = Arc::new(PostgresQueue::new(db.clone()));

        // Create the S3 Client
        let s3_client = create_s3_client().await;

        Ok(Self {
            config,
            db,
            queue,
            s3_client,
        })
    }
}
