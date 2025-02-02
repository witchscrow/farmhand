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

pub async fn sync_directory_to_bucket<P: AsRef<std::path::Path>>(
    client: &Client,
    local_dir: P,
    bucket: &str,
    target_prefix: &str,
    ignore_patterns: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let local_dir = local_dir.as_ref();
    if !local_dir.is_dir() {
        return Err("Source path is not a directory".into());
    }

    let walk_dir = walkdir::WalkDir::new(local_dir);

    for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if ignore_patterns
            .iter()
            .any(|pattern| path.to_string_lossy().contains(pattern))
        {
            tracing::debug!("Skipping {:?}", path);
            continue;
        }
        if path.is_file() {
            let relative_path = path
                .strip_prefix(local_dir)
                .map_err(|e| format!("Failed to strip prefix: {}", e))?;
            let key = if target_prefix.is_empty() {
                relative_path.to_string_lossy().to_string()
            } else {
                format!("{}/{}", target_prefix, relative_path.to_string_lossy())
            };

            let body = aws_sdk_s3::primitives::ByteStream::from_path(path).await?;
            tracing::debug!("full key from walkdir {key}");
            client
                .put_object()
                .bucket(bucket)
                .key(&key)
                .body(body)
                .send()
                .await?;
        }
    }

    Ok(())
}
