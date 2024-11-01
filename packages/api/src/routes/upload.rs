use axum::{
    extract::{Multipart, State},
    routing::post,
    Extension, Router,
};
use db::users::User;
use std::time::Duration;
use std::{path::Path, sync::Arc};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::AppState; // Note the AsyncWriteExt trait

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10GB limit

async fn upload_video(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<String, String> {
    while let Some(mut field) = multipart.next_field().await.map_err(|e| e.to_string())? {
        // Content type validation
        if !field
            .content_type()
            .map(|ct| ct.starts_with("video/"))
            .unwrap_or(false)
        {
            return Err("Invalid content type".to_string());
        }

        let name = field.name().unwrap_or("").to_string();
        let filename = field.file_name().ok_or("No filename provided")?.to_string();

        // Validate file type
        if !filename.ends_with(".mp4") && !filename.ends_with(".mov") {
            return Err("Invalid file type".to_string());
        }

        // Create temporary file
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(&filename);
        let file = File::create(&temp_path).await.map_err(|e| e.to_string())?;
        let mut writer = tokio::io::BufWriter::new(file);

        // Track progress and rate limiting
        let mut total_bytes = 0u64;
        let mut last_progress_report = tokio::time::Instant::now();
        let mut last_chunk_time = tokio::time::Instant::now();

        // Process upload in chunks
        while let Some(chunk_result) = field.chunk().await.map_err(|e| e.to_string())? {
            // Rate limiting
            let now = tokio::time::Instant::now();
            if now.duration_since(last_chunk_time) < Duration::from_millis(100) {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            last_chunk_time = now;

            // Process chunk
            total_bytes += chunk_result.len() as u64;
            if total_bytes > MAX_FILE_SIZE {
                // Clean up on error
                tokio::fs::remove_file(&temp_path).await.ok();
                return Err("File too large".to_string());
            }

            // Progress reporting (every 5MB)
            if total_bytes - (last_progress_report.elapsed().as_secs() as u64 * CHUNK_SIZE as u64)
                > 5 * CHUNK_SIZE as u64
            {
                println!(
                    "Upload progress: {:.2} MB",
                    total_bytes as f64 / 1_048_576.0
                );
                last_progress_report = tokio::time::Instant::now();
            }

            writer
                .write_all(&chunk_result)
                .await
                .map_err(|e| e.to_string())?;
        }

        // Ensure all data is written
        writer.flush().await.map_err(|e| e.to_string())?;

        // Move file to final location
        let final_path = Path::new("./uploads").join(&filename);
        tokio::fs::rename(&temp_path, &final_path)
            .await
            .map_err(|e| e.to_string())?;

        return Ok(format!(
            "Successfully uploaded: {} ({:.2} MB)",
            filename,
            total_bytes as f64 / 1_048_576.0
        ));
    }

    Err("No file found in request".to_string())
}

// Error handler for cleanup
fn cleanup_on_error(path: &Path) {
    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
}
