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

#[derive(Deserialize, Debug)]
pub struct VideoByID {
    id: String,
}

#[derive(Deserialize, Debug)]
pub struct VideoByUserName {
    name: String,
}

#[derive(Serialize)]
pub struct SanitizedVideoData {
    id: String,
    title: String,
    processing_status: ProcessingStatus,
    video_path: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
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
    tracing::debug!(
        "Got query params:\n\tvideo_query: {:?}\n\tusername_query: {:?}",
        video_query,
        username_query
    );
    match (video_query, username_query) {
        // Video by ID
        (Some(video_query), None) => {
            let video = Video::by_id(&state.db, &video_query.id)
                .await
                .map_err(|e| StatusCode::BAD_REQUEST)?;
            if let Some(video) = video {
                let video = SanitizedVideoData {
                    id: video.id,
                    processing_status: video.processing_status,
                    title: video.title,
                    video_path: video.processed_video_path,
                    created_at: video.created_at,
                    updated_at: video.updated_at,
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
                        id: video.id,
                        title: video.title,
                        processing_status: video.processing_status,
                        video_path: video.processed_video_path,
                        created_at: video.created_at,
                        updated_at: video.updated_at,
                    })
                    .collect();
                Ok(Json(VideoResponse { videos }))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        // Videos by video id and user name
        (Some(video_query), Some(username_query)) => Err(StatusCode::NOT_IMPLEMENTED),
        // No query params, get all videos
        (None, None) => {
            tracing::debug!("No queries provided, getting all videos");
            let videos = match Video::all(&state.db).await {
                Ok(videos) => videos,
                Err(e) => {
                    tracing::error!("Error listing videos: {e}");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };

            if !videos.is_empty() {
                let videos = videos
                    .into_iter()
                    .map(|video| SanitizedVideoData {
                        id: video.id,
                        title: video.title,
                        processing_status: video.processing_status,
                        video_path: video.processed_video_path,
                        created_at: video.created_at,
                        updated_at: video.updated_at,
                    })
                    .collect();
                Ok(Json(VideoResponse { videos }))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
    }
}
