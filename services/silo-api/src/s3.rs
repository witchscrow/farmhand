use aws_config::Region;
use aws_sdk_s3::Client;

/// Create an S3 Client configured against Cloudflare R2
pub async fn create_s3_client() -> Client {
    let region = Region::new("auto");
    let r2_account_id = std::env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID required");
    let endpoint_url = format!("https://{}.r2.cloudflarestorage.com", r2_account_id);

    let config = aws_config::from_env()
        .region(region)
        .endpoint_url(endpoint_url)
        .load()
        .await;

    Client::new(&config)
}
