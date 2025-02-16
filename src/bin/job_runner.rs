use anyhow::Result;
use async_nats::jetstream::AckKind;
use farmhand::workers::{self, runner::process_message};
use futures::StreamExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Connect to the stream
    tracing::debug!("Connecting to NATS server");
    let nats_client = workers::create_nats_client().await?;
    let jq_name = "FARMHAND_JOBS".to_string();
    tracing::debug!("Connecting to queue");
    let queue = workers::Queue::connect(jq_name, nats_client)
        .await
        .expect("Failed to create worker queue");

    // Create a consumer for the queue
    let subject = "farmhand_jobs.>".to_string(); // All jobs
    let runner_name = "farmhand_runner_1".to_string();
    tracing::info!("Listening for jobs {} on {}", subject, runner_name);
    let consumer = queue.create_consumer(Some(runner_name), subject).await?;

    // Start consuming jobs
    loop {
        let mut jobs = consumer.fetch().max_messages(3).messages().await?;

        while let Some(job) = jobs.next().await {
            // Make sure the job is good to go
            let Ok(job) = job else {
                tracing::error!("Failed to receive job");
                continue;
            };
            // Process the message itself, ack on success, nack on failure
            match process_message(&job.message).await {
                Ok(_) => job.ack().await.expect("Failed to ack job"),
                Err(err) => {
                    tracing::error!("Failed to process job: {}", err);
                    job.ack_with(AckKind::Nak(None))
                        .await
                        .expect("Failed to nack job");
                }
            }
        }

        // Optional: Add a small delay to prevent tight loops when there are no jobs
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
