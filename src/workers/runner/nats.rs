use anyhow::Result;

/// Get the NATS URL from the environment variable or default to "nats://localhost:4222"
pub fn get_nats_url() -> String {
    std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string())
}

/// Create a NATS client using the provided URL
pub async fn create_nats_client() -> Result<async_nats::Client> {
    let url = get_nats_url();
    async_nats::connect(url)
        .await
        .map_err(|e| anyhow::Error::new(e))
}
