use anyhow::Result;
use farmhand::{db, workers};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "down=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::warn!("Deleting all data from the project, this is a destructive operation");
    for i in (1..=5).rev() {
        tracing::warn!("Deleting all data in {} seconds...", i);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    tracing::info!("Starting deletion process");
    let db_pool = db::connect_to_database().await?;

    // Delete all data from the database
    tracing::debug!("Deleting all data from the database");
    db::delete_all_data(&db_pool).await?;

    tracing::info!("Successfully deleted all data from the database");

    // Delete all streams
    tracing::debug!("Deleting all streams");
    let nats_client = workers::create_nats_client().await?;
    let jq_name = "FARMHAND_JOBS".to_string();
    workers::Queue::delete(jq_name, nats_client).await?;

    tracing::info!("Successfully deleted all streams");
    Ok(())
}
