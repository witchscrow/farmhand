use db::connect_to_database;
use queue::{runner, PostgresQueue, Queue};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start the tracer
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "forge=debug,queue=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Spawn a health check server
    tokio::spawn(async {
        let app = axum::Router::new().route("/health", axum::routing::get(|| async { "OK" }));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        tracing::info!("Health check server listening on port 8080");
        axum::serve(listener, app).await.unwrap();
    });

    // Get database connection pool using the db package
    let db_pool = connect_to_database().await?;

    // Create queue instance
    let queue = PostgresQueue::new(db_pool.clone());
    let queue = Arc::new(queue) as Arc<dyn Queue>;

    // Number of concurrent jobs to process
    let concurrency = 3;

    tracing::info!("Starting queue worker with concurrency {}", concurrency);

    // Run the worker
    runner::run_worker(queue, concurrency, &db_pool).await;

    Ok(())
}
