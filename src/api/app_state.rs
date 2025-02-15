use super::config::Config;
use crate::{db::connect_to_database, storage::s3::create_s3_client};
use sqlx::PgPool;

/// Shared state available to the API
pub struct AppState {
    pub db: PgPool,
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

        Ok(Self {
            config,
            db,
            s3_client,
        })
    }
}
