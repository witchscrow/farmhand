use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Extension,
};
use db::{users::User, Video};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicU64, Ordering};
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncSeekExt, AsyncWriteExt, BufWriter},
    sync::Mutex,
};
use uuid::Uuid;

use crate::AppState;

pub const UPLOAD_CHUNK_SIZE: usize = 50 * 1024 * 1024; // 50MB chunks
pub const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10GB limit
const BUFFER_SIZE: usize = UPLOAD_CHUNK_SIZE * 4; // 200MB buffer
const MP4_BUFFER_SIZE: usize = 10 * 1024 * 1024; // 10MB buffer for MP4 files

#[derive(Debug)]
struct UploadState {
    file: BufWriter<File>,
    chunks_received: AtomicU64,
    total_chunks: u64,
    temp_path: String,
    final_path: String,
    is_mp4: bool,
    write_position: AtomicU64,
}

lazy_static::lazy_static! {
    static ref UPLOAD_STATES: Mutex<HashMap<String, UploadState>> = Mutex::new(HashMap::new());
}

pub async fn upload_video(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    mut multipart: Multipart,
) -> Result<String, StatusCode> {
    let user = match user {
        Some(user) => user,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let mut filename = String::new();
    let mut chunk_index = 0u64;
    let mut total_bytes = 0u64;
    let mut client_checksum = String::new();
    let mut total_size = 0u64;

    // Create uploads directory if it doesn't exist
    let upload_dir = std::env::current_dir()
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?
        .join("uploads");
    tokio::fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::debug!("Processing multipart upload");

    // Get chunk index
    if let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Error reading chunk index: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        if field.name() != Some("chunkIndex") {
            tracing::error!("Expected chunkIndex, got {:?}", field.name());
            return Err(StatusCode::BAD_REQUEST);
        }
        chunk_index = field
            .text()
            .await
            .map_err(|e| StatusCode::BAD_REQUEST)?
            .parse::<u64>()
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Get total file size
    if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| StatusCode::BAD_REQUEST)?
    {
        if field.name() != Some("totalSize") {
            tracing::error!("Missing total size");
            return Err(StatusCode::BAD_REQUEST);
        }
        total_size = field
            .text()
            .await
            .map_err(|e| StatusCode::BAD_REQUEST)?
            .parse::<u64>()
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Get checksum
    if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| StatusCode::BAD_REQUEST)?
    {
        if field.name() != Some("checksum") {
            tracing::error!("Missing checksum");
            return Err(StatusCode::BAD_REQUEST);
        }
        client_checksum = field.text().await.map_err(|e| StatusCode::BAD_REQUEST)?;
    }

    // Get file chunk
    if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| StatusCode::BAD_REQUEST)?
    {
        // Content type validation
        if !field
            .content_type()
            .map(|ct| ct.starts_with("video/"))
            .unwrap_or(false)
        {
            tracing::error!("Invalid content type in upload");
            return Err(StatusCode::BAD_REQUEST);
        }

        filename = field
            .file_name()
            .ok_or(StatusCode::BAD_REQUEST)?
            .to_string();
        let is_mp4 = filename.to_lowercase().ends_with(".mp4");

        tracing::debug!("Processing file: {} (chunk {})", filename, chunk_index);

        // Validate file type
        if !filename.ends_with(".mp4") && !filename.ends_with(".mov") && !filename.ends_with(".m4v")
        {
            tracing::error!("Invalid file type in upload");
            return Err(StatusCode::BAD_REQUEST);
        }

        let final_path = upload_dir.join(&filename);
        let temp_path = upload_dir.join(format!("{}.temp", Uuid::new_v4()));

        // Read the field data
        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        // Verify checksum
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let server_checksum = hex::encode(hasher.finalize());

        if server_checksum != client_checksum {
            tracing::error!("Checksum mismatch for chunk {}", chunk_index);
            return Err(StatusCode::BAD_REQUEST);
        }

        total_bytes = data.len() as u64;

        if total_size > MAX_FILE_SIZE {
            tracing::error!("File too large: {}", total_size);
            return Err(StatusCode::BAD_REQUEST);
        }

        {
            let mut upload_states = UPLOAD_STATES.lock().await;
            let upload_state = if chunk_index == 0 {
                let mut file = OpenOptions::new()
                    .create(true)
                    .read(true)
                    .write(true)
                    .truncate(true)
                    .open(&temp_path)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                file.set_len(total_size)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                file.seek(std::io::SeekFrom::Start(0))
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let buffer_size = if is_mp4 { MP4_BUFFER_SIZE } else { BUFFER_SIZE };

                let file = BufWriter::with_capacity(buffer_size, file);
                let total_chunks =
                    (total_size + UPLOAD_CHUNK_SIZE as u64 - 1) / UPLOAD_CHUNK_SIZE as u64;

                let state = UploadState {
                    file,
                    chunks_received: AtomicU64::new(0),
                    total_chunks,
                    temp_path: temp_path.to_string_lossy().into_owned(),
                    final_path: final_path.to_string_lossy().into_owned(),
                    is_mp4,
                    write_position: AtomicU64::new(0),
                };

                upload_states.insert(filename.clone(), state);
                upload_states.get_mut(&filename).unwrap()
            } else {
                upload_states
                    .get_mut(&filename)
                    .ok_or(StatusCode::BAD_REQUEST)?
            };

            // Write chunk
            let write_position = chunk_index * UPLOAD_CHUNK_SIZE as u64;
            upload_state
                .file
                .seek(std::io::SeekFrom::Start(write_position))
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            upload_state
                .file
                .write_all(&data)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            if upload_state.is_mp4 {
                if chunk_index % 3 == 0 {
                    upload_state
                        .file
                        .flush()
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                }
            }

            upload_state.chunks_received.fetch_add(1, Ordering::SeqCst);
            upload_state
                .write_position
                .store(write_position + data.len() as u64, Ordering::SeqCst);

            let is_complete =
                upload_state.chunks_received.load(Ordering::SeqCst) >= upload_state.total_chunks;

            if is_complete {
                upload_state
                    .file
                    .flush()
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                upload_state
                    .file
                    .get_ref()
                    .sync_all()
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                // Clone paths before any other operations
                let temp_path_str = upload_state.temp_path.clone();
                let final_path_str = upload_state.final_path.clone();

                // Remove the state from the HashMap before dropping
                upload_states.remove(&filename);

                // Drop the lock
                drop(upload_states);

                // Add a slightly longer delay for MP4 files
                if is_mp4 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                } else {
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }

                // Verify the temp file size before renaming
                match tokio::fs::metadata(&temp_path_str).await {
                    Ok(metadata) => {
                        let temp_size = metadata.len();
                        if temp_size != total_size {
                            tracing::error!(
                                "Size mismatch: expected {}, got {} for file {}",
                                total_size,
                                temp_size,
                                &temp_path_str
                            );
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    }
                    Err(_) => {
                        tracing::error!("Failed to get temp file metadata");
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }

                // Ensure the final path doesn't exist before renaming
                if Path::new(&final_path_str).exists() {
                    match tokio::fs::remove_file(&final_path_str).await {
                        Ok(_) => {
                            tracing::debug!(
                                "Successfully removed existing file at {}",
                                &final_path_str
                            );
                        }
                        Err(e) => {
                            tracing::error!("Failed to remove existing file: {}", e);
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    }
                }

                // Clone the paths again for the spawn_blocking closure
                let temp_path_clone = temp_path_str.clone();
                let final_path_clone = final_path_str.clone();

                // Use std::fs for the rename operation to ensure it's more reliable
                match tokio::task::spawn_blocking(move || {
                    std::fs::rename(&temp_path_clone, &final_path_clone)
                })
                .await
                {
                    Ok(rename_result) => {
                        if let Err(e) = rename_result {
                            tracing::error!(
                                "Failed to rename file from {} to {}: {}",
                                &temp_path_str,
                                &final_path_str,
                                e
                            );
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to execute rename operation: {}", e);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }

                // Verify final file exists and has correct size
                match tokio::fs::metadata(&final_path_str).await {
                    Ok(metadata) => {
                        let final_size = metadata.len();
                        if final_size != total_size {
                            tracing::error!(
                                "Final size mismatch: expected {}, got {} for file {}",
                                total_size,
                                final_size,
                                &final_path_str
                            );
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to verify final file: {}", e);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }

                // Save video metadata to database
                let video_title = filename
                    .trim_end_matches(".mp4")
                    .trim_end_matches(".mov")
                    .trim_end_matches(".m4v")
                    .to_string();

                match Video::create(&state.db, user.id, video_title, final_path_str.clone()).await {
                    Ok(video) => {
                        tracing::debug!("Saved video metadata to database: {:?}", video);

                        // Create and push video processing job to queue
                        let process_video_message =
                            queue::queue::Message::ProcessRawVideoIntoStream {
                                video_id: video.id.to_string(),
                            };

                        if let Err(e) = state
                            .queue
                            .push(
                                process_video_message,
                                None, // Schedule for immediate processing
                            )
                            .await
                        {
                            tracing::error!("Failed to queue video processing job: {}", e);
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }

                        tracing::debug!("Successfully queued video processing job");
                    }
                    Err(e) => {
                        tracing::error!("Failed to save video metadata: {}", e);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
            }
        }

        Ok(format!(
            "Successfully processed chunk {} for: {} ({:.2} MB)",
            chunk_index + 1,
            filename,
            total_bytes as f64 / 1_048_576.0
        ))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

async fn cleanup_temp_files() -> Result<(), std::io::Error> {
    let upload_dir = std::env::current_dir()?.join("uploads");
    let mut read_dir = tokio::fs::read_dir(&upload_dir).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "temp") {
            if let Ok(metadata) = entry.metadata().await {
                if let Ok(modified) = metadata.modified() {
                    if modified.elapsed().unwrap_or_default().as_secs() > 3600 {
                        tracing::debug!("Cleaning up stale temp file: {:?}", path);
                        let _ = tokio::fs::remove_file(&path).await;
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn init_cleanup() {
    tokio::spawn(async {
        loop {
            if let Err(e) = cleanup_temp_files().await {
                tracing::error!("Error cleaning up temp files: {}", e);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        }
    });
}
