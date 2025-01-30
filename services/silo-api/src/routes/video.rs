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
    tracing::trace!(
        "Got video get query params:\n\tvideo_query: {:?}\n\tusername_query: {:?}",
        video_query,
        username_query
    );
    match (video_query, username_query) {
        // Video by ID
        (Some(video_query), None) => {
            let video = Video::by_id(&state.db, &video_query.id)
                .await
                .map_err(|e| StatusCode::BAD_REQUEST)?;
            let res_video = SanitizedVideoData {
                id: video.id,
                processing_status: video.processing_status,
                title: video.title,
                video_path: video.processed_video_path,
                created_at: video.created_at,
                updated_at: video.updated_at,
            };
            Ok(Json(VideoResponse {
                videos: vec![res_video],
            }))
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
        }
    }
}

#[derive(Serialize)]
pub struct DeleteVideoResponse {
    deleted_videos: Vec<String>,
}

// TODO: Refactor to delete from R2 instead
pub async fn delete_videos(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    video_query: Option<Query<VideoByID>>,
) -> impl IntoResponse {
    tracing::debug!(
        "Got video delete query params:\n\tvideo_query: {:?}",
        video_query,
    );
    let user = match user {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let video_ids: Vec<String> = match video_query {
        Some(video_query) => video_query
            .id
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let videos = match db::Video::by_ids(&state.db, &video_ids).await {
        Ok(videos) => videos,
        Err(_e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mut successfully_deleted_ids = Vec::new();

    // Delete the physical video files
    for video in &videos {
        // Only allow deletion if the user owns the video
        if video.user_id != user.id {
            tracing::warn!(
                "User {} attempted to delete video {} owned by {}",
                user.id,
                video.id,
                video.user_id
            );
            continue; // Skip this video and continue with others
        }

        let mut deletion_successful = true;

        // Delete raw video file
        tracing::debug!(
            "Attempting to delete raw video file {}",
            video.raw_video_path
        );
        if let Err(e) = tokio::fs::remove_file(&video.raw_video_path).await {
            if e.kind() == std::io::ErrorKind::NotFound {
                tracing::debug!("Raw video file {} already deleted", video.raw_video_path);
            } else {
                tracing::error!(
                    "Failed to delete raw video file {}: {}",
                    video.raw_video_path,
                    e
                );
                deletion_successful = false;
            }
        } else {
            tracing::debug!(
                "Successfully deleted raw video file {}",
                video.raw_video_path
            );
        }

        // For the processed video folder:
        if let Some(processed_path) = &video.processed_video_path {
            tracing::debug!("Attempting to delete processed video folder");
            let path = std::path::PathBuf::from(processed_path);

            if let Some(parent_dir) = path.parent() {
                if let Some(folder_name) = parent_dir.file_name() {
                    if let Some(folder_str) = folder_name.to_str() {
                        if folder_str == video.id {
                            if let Err(e) = tokio::fs::remove_dir_all(parent_dir).await {
                                if e.kind() == std::io::ErrorKind::NotFound {
                                    tracing::debug!(
                                        "Processed video folder {} already deleted",
                                        parent_dir.display()
                                    );
                                } else {
                                    tracing::error!(
                                        "Failed to delete processed video folder {}: {}",
                                        parent_dir.display(),
                                        e
                                    );
                                    deletion_successful = false;
                                }
                            } else {
                                tracing::debug!(
                                    "Successfully deleted processed video folder {}",
                                    parent_dir.display()
                                );
                            }
                        } else {
                            tracing::error!(
                                "Video folder name '{}' doesn't match video ID '{}'. Skipping deletion.",
                                folder_str,
                                video.id
                            );
                            deletion_successful = false;
                        }
                    } else {
                        tracing::error!("Invalid folder name encoding");
                        deletion_successful = false;
                    }
                } else {
                    tracing::error!("Couldn't get folder name from path");
                    deletion_successful = false;
                }
            } else {
                tracing::error!("Couldn't get parent directory from path");
                deletion_successful = false;
            }
        }

        // Only add to successfully deleted list if all files were deleted
        if deletion_successful {
            successfully_deleted_ids.push(video.id.clone());
        }
    }

    // Only delete videos from database if we successfully deleted their files
    if !successfully_deleted_ids.is_empty() {
        match db::Video::delete(&state.db, user.id, successfully_deleted_ids.clone()).await {
            Ok(_) => Ok(Json(DeleteVideoResponse {
                deleted_videos: successfully_deleted_ids,
            })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        // No videos were successfully deleted
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
