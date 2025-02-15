use std::sync::Arc;

use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use axum::{extract::State, Extension, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    api::app_state::AppState,
    db::{User, Video},
    prelude::get_storage_dir,
};

#[derive(Deserialize)]
pub struct InitUploadRequest {
    parts: i32,
    key: String,
    content_type: String,
    title: Option<String>,
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
    video_id: String,
    key: String,
    part_urls: Vec<PartUrl>,
}

/// Initializes a multipart upload to R2
pub async fn init_upload(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    Json(request): Json<InitUploadRequest>,
) -> Result<Json<InitUploadResponse>, StatusCode> {
    // User required
    tracing::trace!("Checking for user authorization on upload");
    let user = user.ok_or(StatusCode::UNAUTHORIZED)?;
    // First, let R2 know we've completed the upload
    tracing::trace!("Grabbing bucket");
    let bucket = state
        .config
        .upload_bucket
        .clone()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    tracing::trace!("Bucket found {}", &bucket);
    let video_id = Video::gen_id();
    // Get the file extension from the original key
    let extension = request.key.split('.').last().unwrap_or("");
    let storage_root = get_storage_dir();
    let storage_path = format!("{}/{}", storage_root, video_id);
    let key = format!("{}/raw.{}", storage_path, extension);
    tracing::debug!("Full parsed key: {key}");
    // Start multipart upload on R2's side
    let start_upload_output = state
        .s3_client
        .create_multipart_upload()
        .bucket(&bucket)
        .key(&key)
        .content_type(&request.content_type)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Could not start multipart upload {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Parse the upload ID out of the R2 output for downstream use while uploading
    let upload_id = start_upload_output
        .upload_id()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    // Generate presigned links for each part
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
            .key(&key)
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

    // Initialize the video in the database
    let video = Video::create(
        &state.db,
        Some(video_id),
        user.id,
        request.title.unwrap_or("Untitled".to_string()),
        Some(key.clone()),
    )
    .await
    .map_err(|e| {
        tracing::error!("Could not initialize video in database {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(InitUploadResponse {
        upload_id,
        part_urls,
        key,
        video_id: video.id,
    }))
}

#[derive(Deserialize)]
pub struct Parts {
    etag: String,
    number: i32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CompleteUploadRequest {
    upload_id: String,
    video_id: String,
    key: String,
    completed_parts: Vec<Parts>,
}

// TODO: Write route for completing multipart upload
/// Completes a multipart upload to R2
pub async fn complete_upload(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    Json(request): Json<CompleteUploadRequest>,
) -> Result<StatusCode, StatusCode> {
    // User required
    tracing::trace!("Checking for user authorization on upload");
    let _user = user.ok_or(StatusCode::UNAUTHORIZED)?;
    // First, let R2 know we've completed the upload
    tracing::trace!("Grabbing bucket");
    let bucket = state
        .config
        .upload_bucket
        .clone()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    tracing::trace!("Bucket found {}", bucket);

    let serialized_completed_parts = request
        .completed_parts
        .iter()
        .map(|part| {
            CompletedPart::builder()
                .e_tag(part.etag.clone())
                .part_number(part.number)
                .build()
        })
        .collect();

    let completed_upload = CompletedMultipartUpload::builder()
        .set_parts(Some(serialized_completed_parts))
        .build();

    let _completed_parts = state
        .s3_client
        .complete_multipart_upload()
        .bucket(&bucket)
        .key(&request.key)
        .upload_id(&request.upload_id)
        .multipart_upload(completed_upload)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Could not complete multipart upload {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::ACCEPTED)
}
