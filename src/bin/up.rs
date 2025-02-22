use anyhow::Result;
use farmhand::{
    db,
    workers::{
        self,
        events::{EVENT_PREFIX, PRIMARY_STREAM},
    },
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "up=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("Initializing project");
    // Run database-related initialization tasks
    let (_db_handle, _nats_handle) = tokio::join!(init_project_db(), init_project_nats());

    Ok(())
}

/// Function for initializing project-wide database dependencies
async fn init_project_db() {
    tracing::debug!("Starting database initialization");

    // Connect to the database
    tracing::debug!("Connecting to database");
    let db_pool = db::connect_to_database()
        .await
        .expect("Failed to connect to database");

    // Run migrations so we can use the database
    tracing::debug!("Running migrations");
    db::run_migrations(&db_pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Successfully initialized database");
}

/// Function for initializing project-wide nats dependencies
async fn init_project_nats() {
    tracing::debug!("Starting NATS initialization");

    // Connect to the NATS server
    tracing::debug!("Connecting to NATS server");
    let nats_client = workers::create_nats_client()
        .await
        .expect("Failed to connect to NATS");

    // Create the job queue stream
    let jq_desc = Some("All Farmhand events".to_string());
    let all_events = format!("{}.>", EVENT_PREFIX);
    let jq_subjects = vec![all_events];
    workers::Queue::new(
        PRIMARY_STREAM.to_string(),
        jq_desc,
        jq_subjects,
        nats_client,
    )
    .await
    .expect("Failed to create worker queue");

    tracing::info!("Successfully initialized NATS worker queue");
}
