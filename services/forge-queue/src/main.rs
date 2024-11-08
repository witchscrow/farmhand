use db::get_db_pool;
use queue::{PostgresQueue, Queue};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging with environment-based filtering
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Get database connection pool using the db package
    let db_pool = get_db_pool().await?;

    // Create queue instance
    let queue = PostgresQueue::new(db_pool.clone());
    let queue = Arc::new(queue) as Arc<dyn Queue>;

    // Number of concurrent jobs to process
    let concurrency = 3;

    tracing::info!("Starting queue worker with concurrency {}", concurrency);

    // Run the worker
    queue::runner::run_worker(queue, concurrency, &db_pool).await;

    Ok(())
}
