use axum::{
    extract::{Multipart, State},
    Extension,
};
use db::{users::User, Video};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicU64, Ordering};
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufWriter},
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
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<String, String> {
    let mut filename = String::new();
    let mut chunk_index = 0u64;
    let mut total_bytes = 0u64;
    let mut client_checksum = String::new();
    let mut total_size = 0u64;

    // Create uploads directory if it doesn't exist
    let upload_dir = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("uploads");
    tokio::fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| e.to_string())?;

    tracing::debug!("Processing multipart upload");

    // Get chunk index
    if let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Error reading chunk index: {}", e);
        e.to_string()
    })? {
        if field.name() != Some("chunkIndex") {
            return Err(format!("Expected chunkIndex, got {:?}", field.name()));
        }
        chunk_index = field
            .text()
            .await
            .map_err(|e| e.to_string())?
            .parse::<u64>()
            .map_err(|e| e.to_string())?;
    }

    // Get total file size
    if let Some(field) = multipart.next_field().await.map_err(|e| e.to_string())? {
        if field.name() != Some("totalSize") {
            return Err("Missing total size".to_string());
        }
        total_size = field
            .text()
            .await
            .map_err(|e| e.to_string())?
            .parse::<u64>()
            .map_err(|e| e.to_string())?;
    }

    // Get checksum
    if let Some(field) = multipart.next_field().await.map_err(|e| e.to_string())? {
        if field.name() != Some("checksum") {
            return Err("Missing checksum".to_string());
        }
        client_checksum = field.text().await.map_err(|e| e.to_string())?;
    }

    // Get file chunk
    if let Some(field) = multipart.next_field().await.map_err(|e| e.to_string())? {
        // Content type validation
        if !field
            .content_type()
            .map(|ct| ct.starts_with("video/"))
            .unwrap_or(false)
        {
            return Err("Invalid content type".to_string());
        }

        filename = field.file_name().ok_or("No filename provided")?.to_string();
        let is_mp4 = filename.to_lowercase().ends_with(".mp4");

        tracing::debug!("Processing file: {} (chunk {})", filename, chunk_index);

        // Validate file type
        if !filename.ends_with(".mp4") && !filename.ends_with(".mov") && !filename.ends_with(".m4v")
        {
            return Err("Invalid file type".to_string());
        }

        let final_path = upload_dir.join(&filename);
        let temp_path = upload_dir.join(format!("{}.temp", Uuid::new_v4()));

        // Read the field data
        let data = field.bytes().await.map_err(|e| e.to_string())?;

        // Verify checksum
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let server_checksum = hex::encode(hasher.finalize());

        if server_checksum != client_checksum {
            return Err(format!("Checksum mismatch for chunk {}", chunk_index));
        }

        total_bytes = data.len() as u64;

        if total_size > MAX_FILE_SIZE {
            return Err("File too large".to_string());
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
                    .map_err(|e| e.to_string())?;

                // Pre-allocate space for all files
                file.set_len(total_size).await.map_err(|e| e.to_string())?;

                file.seek(std::io::SeekFrom::Start(0))
                    .await
                    .map_err(|e| e.to_string())?;

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
                    .ok_or("Upload not initialized")?
            };

            // Write chunk
            let write_position = chunk_index * UPLOAD_CHUNK_SIZE as u64;
            upload_state
                .file
                .seek(std::io::SeekFrom::Start(write_position))
                .await
                .map_err(|e| e.to_string())?;

            upload_state
                .file
                .write_all(&data)
                .await
                .map_err(|e| e.to_string())?;

            // Only flush every few chunks for MP4 files
            if upload_state.is_mp4 {
                if chunk_index % 3 == 0 {
                    upload_state.file.flush().await.map_err(|e| {
                        tracing::error!("Error flushing chunk: {}", e);
                        e.to_string()
                    })?;
                }
            }

            upload_state.chunks_received.fetch_add(1, Ordering::SeqCst);
            upload_state
                .write_position
                .store(write_position + data.len() as u64, Ordering::SeqCst);

            let is_complete =
                upload_state.chunks_received.load(Ordering::SeqCst) >= upload_state.total_chunks;

            if is_complete {
                // Ensure all data is written and flushed
                upload_state.file.flush().await.map_err(|e| {
                    tracing::error!("Error flushing file: {}", e);
                    e.to_string()
                })?;

                // Get the inner file to sync
                upload_state.file.get_ref().sync_all().await.map_err(|e| {
                    tracing::error!("Error syncing file: {}", e);
                    e.to_string()
                })?;

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
                let temp_size = match tokio::fs::metadata(&temp_path_str).await {
                    Ok(metadata) => metadata.len(),
                    Err(e) => {
                        tracing::error!("Failed to get temp file metadata: {}", e);
                        return Err(format!("Failed to verify temp file: {}", e));
                    }
                };

                if temp_size != total_size {
                    tracing::error!(
                        "Size mismatch: expected {}, got {} for file {}",
                        total_size,
                        temp_size,
                        &temp_path_str
                    );
                    return Err(format!(
                        "File size mismatch: expected {}, got {}",
                        total_size, temp_size
                    ));
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
                            return Err(format!("Failed to remove existing file: {}", e));
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
                            return Err(format!("Failed to rename file: {}", e));
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to execute rename operation: {}", e);
                        return Err(format!("Failed to execute rename operation: {}", e));
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
                            return Err(format!(
                                "Final file size mismatch: expected {}, got {}",
                                total_size, final_size
                            ));
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to verify final file: {}", e);
                        return Err(format!("Failed to verify final file: {}", e));
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
                    }
                    Err(e) => {
                        tracing::error!("Failed to save video metadata: {}", e);
                        // Consider whether to return an error here or just log it
                        return Err(format!("Failed to save video metadata: {}", e));
                    }
                }

                tracing::debug!("Successfully completed file upload and rename");
            }
        }

        Ok(format!(
            "Successfully processed chunk {} for: {} ({:.2} MB)",
            chunk_index + 1,
            filename,
            total_bytes as f64 / 1_048_576.0
        ))
    } else {
        Err("No file data received".to_string())
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
