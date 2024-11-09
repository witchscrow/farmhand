use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use db::{ProcessingStatus, User, Video};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

#[derive(Deserialize)]
pub struct VideoByID {
    id: String,
}

#[derive(Deserialize)]
pub struct VideoByUserName {
    name: String,
}

#[derive(Serialize)]
pub struct VideoResponse {
    title: String,
    processing_status: ProcessingStatus,
}

/// A function for getting videos based on video id, user id, username, or combinations thereof
pub async fn get_videos(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    video_query: Option<Query<VideoByID>>,
    username_query: Option<Query<VideoByUserName>>,
) -> impl IntoResponse {
    match (video_query, username_query) {
        // Video by ID
        (Some(video_query), None) => {
            let video = Video::by_id(&state.db, &video_query.id)
                .await
                .map_err(|e| StatusCode::BAD_REQUEST)?;
            if let Some(video) = video {
                Ok(Json(VideoResponse {
                    processing_status: video.processing_status,
                    title: video.title,
                }))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        // Videos by user name
        (None, Some(username_query)) => {
            Err(StatusCode::NOT_IMPLEMENTED)
            // TODO: Check if user requesting has access to the user videos requested
            // TODO: Check the privacy status, guest users can get all public videos
        }
        // Videos by video id and user name
        (Some(video_query), Some(username_query)) => Err(StatusCode::NOT_IMPLEMENTED),
        // No query params
        (None, None) => Err(StatusCode::NOT_IMPLEMENTED),
    }
}
