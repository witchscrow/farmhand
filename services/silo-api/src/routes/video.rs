use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension,
};
use db::User;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

#[derive(Deserialize)]
pub struct VideoByUser {
    user_id: Uuid,
}

#[derive(Deserialize)]
pub struct VideoByID {
    id: String,
}

/// A function for getting videos based on video id, user id, or both
pub async fn get_videos(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    video_query: Option<Query<VideoByID>>,
    user_query: Option<Query<VideoByUser>>,
) -> impl IntoResponse {
    match (video_query, user_query) {
        (Some(video_query), None) => {
            format!("Get video with ID: {}", video_query.id).into_response()
            // TODO: Get the video requested
            // TODO: Add a "privacy_status" to a video, allowing it to be public and private to start
            // TODO: If a video is public, return all relevant data
            // TODO: If a video is private, validate the user has access
        }
        // Get all videos for a user
        (None, Some(user_query)) => {
            format!("Get videos for user: {}", user_query.user_id).into_response()
            // TODO: Check if user requesting has access to the user videos requested
            // TODO: Check the privacy status, guest users can get all public videos
        }
        (Some(video_query), Some(user_query)) => {
            // Get specific video for specific user
            format!(
                "Get video {} for user {}",
                video_query.id, user_query.user_id
            )
            .into_response()
        }
        (None, None) => {
            // No parameters provided
            "No query parameters provided".into_response()
        }
    }
}
