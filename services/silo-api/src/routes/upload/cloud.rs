use std::sync::Arc;

use axum::{debug_handler, extract::State, Extension, Json};
use db::User;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
pub struct InitUploadRequest {
    parts: i32,
    key: String,
    content_type: String,
}

#[derive(Serialize)]
/// PartUrl of a multipart upload, containing the part_number and url itself
struct PartUrl {
    part_number: i32,
    url: String,
}

#[derive(Serialize)]
pub struct InitUploadResponse {
    upload_id: String,
    part_urls: Vec<PartUrl>,
}

#[debug_handler]
/// Initializes a multipart upload to R2
pub async fn init_upload(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    Json(request): Json<InitUploadRequest>,
) -> Result<Json<InitUploadResponse>, StatusCode> {
    let bucket = state
        .config
        .upload_bucket
        .clone()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let start_upload_output = state
        .s3_client
        .create_multipart_upload()
        .bucket(&bucket)
        .key(&request.key)
        .content_type(&request.content_type)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Could not start multipart upload {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let upload_id = start_upload_output
        .upload_id()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let mut part_urls = Vec::new();
    for part_number in 1..=request.parts {
        // Construct Duration
        let expires_in = std::time::Duration::from_secs(3600);
        // Construct presigning config
        let config =
            aws_sdk_s3::presigning::PresigningConfig::expires_in(expires_in).map_err(|e| {
                tracing::error!("Could not construct presigning config {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        // Generate URL
        let presigned_url = state
            .s3_client
            .upload_part()
            .bucket(&bucket)
            .key(&request.key)
            .upload_id(&upload_id)
            .part_number(part_number)
            .presigned(config)
            .await
            .map_err(|e| {
                tracing::error!("Could not generate presigned url {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        part_urls.push(PartUrl {
            part_number,
            url: presigned_url.uri().to_string(),
        });
    }

    Ok(Json(InitUploadResponse {
        upload_id,
        part_urls,
    }))
}

// TODO: Write route for completing multipart upload
