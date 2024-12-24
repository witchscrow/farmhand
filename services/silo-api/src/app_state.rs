use std::sync::Arc;

use queue::{PostgresQueue, Queue};
use sqlx::PgPool;

use crate::config::Config;

/// Shared state available to the API
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
    pub queue: Arc<dyn Queue>,
    pub s3_client: Option<aws_sdk_s3::Client>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize a connection to the database
        let db = db::connect_to_database()
            .await
            .expect("Could not connect to database");
        // Initialize the queue
        let queue = Arc::new(PostgresQueue::new(db.clone()));

        // If there's S3 credentials, construct an S3 client
        let s3_client = match config.s3_options.clone() {
            None => None,
            Some(options) => Some(aws_sdk_s3::Client::from_conf(options)),
        };

        Ok(Self {
            config,
            db,
            queue,
            s3_client,
        })
    }
}
