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
pub struct VideoByUserID {
    user_id: Uuid,
}

#[derive(Deserialize)]
pub struct VideoByID {
    id: String,
}

#[derive(Deserialize)]
pub struct VideoByUserName {
    name: String,
}

/// A function for getting videos based on video id, user id, username, or combinations thereof
pub async fn get_videos(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    video_query: Option<Query<VideoByID>>,
    user_id_query: Option<Query<VideoByUserID>>,
    username_query: Option<Query<VideoByUserName>>,
) -> impl IntoResponse {
    match (video_query, user_id_query, username_query) {
        (Some(video_query), None, None) => {
            format!("Get video with ID: {}", video_query.id).into_response()
            // TODO: Get the video requested
            // TODO: Add a "privacy_status" to a video, allowing it to be public and private to start
            // TODO: If a video is public, return all relevant data
            // TODO: If a video is private, validate the user has access
        }
        (None, Some(user_query), None) => {
            format!("Get videos for user ID: {}", user_query.user_id).into_response()
            // TODO: Check if user requesting has access to the user videos requested
            // TODO: Check the privacy status, guest users can get all public videos
        }
        (None, None, Some(username_query)) => {
            format!("Get videos for username: {}", username_query.name).into_response()
            // TODO: Check if user requesting has access to the user videos requested
            // TODO: Check the privacy status, guest users can get all public videos
        }
        (Some(video_query), Some(user_query), None) => format!(
            "Get video {} for user ID {}",
            video_query.id, user_query.user_id
        )
        .into_response(),
        (Some(video_query), None, Some(username_query)) => format!(
            "Get video {} for username {}",
            video_query.id, username_query.name
        )
        .into_response(),
        (None, Some(user_query), Some(username_query)) => format!(
            "Get videos for user ID {} and username {}",
            user_query.user_id, username_query.name
        )
        .into_response(),
        (Some(video_query), Some(user_query), Some(username_query)) => format!(
            "Get video {} for user ID {} and username {}",
            video_query.id, user_query.user_id, username_query.name
        )
        .into_response(),
        (None, None, None) => {
            // No parameters provided
            "No query parameters provided".into_response()
        }
    }
}
