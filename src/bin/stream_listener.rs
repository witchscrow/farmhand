//! This is a program for simply logging the stream of events going to farmhand nats
//! It intentionally does not do anything else

use anyhow::Result;
use farmhand::workers::{self, events::MESSAGE_PREFIX};
use futures::StreamExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Create the NATS client
    tracing::debug!("Connecting to NATS server");
    let nats_client = workers::create_nats_client().await?;
    // Setup the Jetstream queue
    let listener = workers::Stream::connect(nats_client)
        .await
        .expect("Failed to create worker queue");

    // Get all events from the stream
    let subject = format!("{}.>", MESSAGE_PREFIX); // All farmhand events
    let runner_name = "farmhand_listener_1".to_string();
    tracing::info!("Listening for events {} on {}", subject, runner_name);
    // Create the consumer to listen for events
    let consumer = listener.create_consumer(Some(runner_name), subject).await?;
    loop {
        let mut jobs = consumer.fetch().max_messages(20).messages().await?;
        while let Some(job) = jobs.next().await {
            // Make sure the job is good to go
            let Ok(job) = job else {
                tracing::error!("Failed to receive job");
                continue;
            };
            tracing::info!("{:?}", job);
        }

        // Add a small delay to prevent tight loops when there are no jobs
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
