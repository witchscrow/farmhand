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
pub struct SanitizedVideoData {
    title: String,
    processing_status: ProcessingStatus,
}

#[derive(Serialize)]
pub struct VideoResponse {
    videos: Vec<SanitizedVideoData>,
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
                let video = SanitizedVideoData {
                    processing_status: video.processing_status,
                    title: video.title,
                };
                Ok(Json(VideoResponse {
                    videos: vec![video],
                }))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        // Videos by user name
        (None, Some(username_query)) => {
            let videos = Video::by_username(&state.db, &username_query.name)
                .await
                .map_err(|e| {
                    tracing::error!("Error getting videos by username {e}");
                    StatusCode::BAD_REQUEST
                })?;

            if !videos.is_empty() {
                let videos = videos
                    .into_iter()
                    .map(|video| SanitizedVideoData {
                        title: video.title,
                        processing_status: video.processing_status,
                    })
                    .collect();
                Ok(Json(VideoResponse { videos }))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        // Videos by video id and user name
        (Some(video_query), Some(username_query)) => Err(StatusCode::NOT_IMPLEMENTED),
        // No query params
        (None, None) => Err(StatusCode::NOT_IMPLEMENTED),
    }
}
