use crate::error::Error;
use crate::job::PostgresJobStatus;
use crate::queue::{Job, Message, Queue};
use db::Video;
use futures::{stream, StreamExt};
use sqlx::{Pool, Postgres};
use std::fs;
use std::io::Write;
use std::{path::PathBuf, sync::Arc, time::Duration};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use vod::stream::{HLSConverter, Quality};
use vod::{DownloadSettings, Vod};
use zip::{write::FileOptions, ZipWriter};

/// Runs a loop that pulls jobs from the queue and runs <concurrency> jobs each loop
pub async fn run_worker(queue: Arc<dyn Queue>, concurrency: usize, db_conn: &Pool<Postgres>) {
    loop {
        // Pulls jobs from the queue
        let jobs = match queue.pull(concurrency as i32).await {
            Ok(jobs) => jobs,
            Err(err) => {
                tracing::error!("runner: error pulling jobs {}", err);
                tokio::time::sleep(Duration::from_millis(500)).await;
                Vec::new()
            }
        };

        let number_of_jobs = jobs.len();
        if number_of_jobs > 0 {
            tracing::debug!("Fetched {} jobs", number_of_jobs);
        }

        stream::iter(jobs)
            .for_each_concurrent(concurrency, |job| async {
                tracing::debug!("Starting job {}", job.id);
                let job_id = job.id;

                let res = match handle_job(queue.clone(), job, db_conn).await {
                    Ok(_) => queue.delete_job(job_id).await,
                    Err(err) => {
                        tracing::error!("run_worker: handling job({}): {}", job_id, &err);
                        queue.fail_job(job_id).await
                    }
                };

                if let Err(err) = res {
                    tracing::error!("run_worker: deleting / failing job: {}", &err);
                }
            })
            .await;

        tokio::time::sleep(Duration::from_millis(125)).await;
    }
}

/// Individually processes a single job, based on its Job message type
async fn handle_job(queue: Arc<dyn Queue>, job: Job, db: &Pool<Postgres>) -> Result<(), Error> {
    tracing::debug!("Got job of type {:?}", &job.message);
    match job.message {
        Message::ProcessRawVideoIntoStream { video_id } => {
            tracing::info!("Start video processing for video_id {video_id}");

            // Update video status to Processing
            sqlx::query(
                "UPDATE videos SET processing_status = 'processing', updated_at = NOW() WHERE id = $1"
            )
            .bind(&video_id)
            .execute(db)
            .await?;

            // Download the video
            let vod = Vod::by_id(db, video_id)
                .await
                .expect("Could not get vod by id");
            let video_storage_folder = PathBuf::from(get_storage_dir());
            let download_settings = DownloadSettings {
                client: common::s3::create_s3_client().await,
                bucket: std::env::var("UPLOAD_BUCKET")
                    .expect("Could not get UPLOAD_BUCKET from environment"),
            };
            let video_path = vod
                .clone()
                .get_raw_video(video_storage_folder, Some(download_settings))
                .await
                .expect("Could not get raw video")
                .expect("No video path found");
            tracing::debug!("Video located at {:?}", video_path);
            // Create output directory
            let output_dir = PathBuf::from(get_storage_dir()).join(&vod.video.id.to_string());
            let ffmpeg_location = get_ffmpeg_location();

            // Initialize HLS converter
            let converter = HLSConverter::new(
                ffmpeg_location.as_str(),
                output_dir
                    .to_str()
                    .expect("Could not convert output dir to path string"),
            )
            .map_err(|e| Error::VideoProcessingError(e.to_string()))?;

            // Define quality levels
            let qualities = vec![
                Quality::new(1920, 1080, "5000k", "1080p"),
                Quality::new(1280, 720, "2800k", "720p"),
                Quality::new(854, 480, "1400k", "480p"),
            ];

            // Process the video
            converter
                .convert_to_hls(&video_path, qualities)
                .map_err(|e| Error::VideoProcessingError(e.to_string()))?;

            // Update with success status
            let master_playlist_path = output_dir.join("master.m3u8");
            sqlx::query(
                "UPDATE videos SET
                    processing_status = 'completed',
                    processed_video_path = $1,
                    updated_at = NOW()
                WHERE id = $2",
            )
            .bind(master_playlist_path.to_str().unwrap())
            .bind(&vod.video.id)
            .execute(db)
            .await?;

            // After completed, queue up a Compress Raw Video job
            let scheduled_time = chrono::Utc::now() + chrono::Duration::days(1);
            queue
                .push(
                    Message::CompressRawVideo {
                        video_id: vod.video.id.clone(),
                    },
                    PostgresJobStatus::Queued,
                    Some(scheduled_time),
                )
                .await?;

            tracing::info!(
                "Successfully processed video {} and queued compression job",
                &vod.video.id
            );
        }
        Message::CompressRawVideo { video_id } => {
            tracing::info!("Start video compression for video_id {video_id}");

            // Update video compression status to compressing
            sqlx::query(
                "UPDATE videos SET compression_status = 'compressing', updated_at = NOW() WHERE id = $1"
            )
            .bind(&video_id)
            .execute(db)
            .await?;

            // Wrap the compression logic in a result to handle failures
            let compression_result = async {
                // Get video details
                let video = sqlx::query_as::<_, Video>("SELECT * FROM videos WHERE id = $1")
                    .bind(&video_id)
                    .fetch_one(db)
                    .await?;

                // Create video-specific directory if it doesn't exist
                let videos_dir = PathBuf::from(get_storage_dir());
                let video_dir = videos_dir.join(&video_id.to_string());
                fs::create_dir_all(&video_dir)
                    .map_err(|e| Error::VideoProcessingError(e.to_string()))?;

                let zip_path = video_dir.join("raw.zip");
                let mut zip = ZipWriter::new(
                    fs::File::create(&zip_path)
                        .map_err(|e| Error::VideoProcessingError(e.to_string()))?,
                );

                let raw_video_path = PathBuf::from(&video.raw_video_path);
                let file_name = raw_video_path
                    .file_name()
                    .ok_or_else(|| {
                        Error::VideoProcessingError("Invalid raw video path".to_string())
                    })?
                    .to_string_lossy()
                    .into_owned();

                zip.start_file(&file_name, FileOptions::default())
                    .map_err(|e| Error::VideoProcessingError(e.to_string()))?;

                // Read the file in chunks
                let mut file = File::open(&raw_video_path)
                    .await
                    .map_err(|e| Error::VideoProcessingError(e.to_string()))?;
                let mut buffer = vec![0; 1024 * 1024]; // 1MB chunks

                loop {
                    let n = file
                        .read(&mut buffer)
                        .await
                        .map_err(|e| Error::VideoProcessingError(e.to_string()))?;
                    if n == 0 {
                        break;
                    }
                    zip.write_all(&buffer[..n])
                        .map_err(|e| Error::VideoProcessingError(e.to_string()))?;
                }

                zip.finish()
                    .map_err(|e| Error::VideoProcessingError(e.to_string()))?;

                // Close file handle before trying to remove
                drop(file);

                // Remove the original raw video file
                tokio::fs::remove_file(&raw_video_path).await.map_err(|e| {
                    Error::VideoProcessingError(format!("Failed to remove raw video: {}", e))
                })?;

                Ok::<PathBuf, Error>(zip_path)
            }
            .await;

            match compression_result {
                Ok(zip_path) => {
                    // Update the video record with success status and compressed file path
                    sqlx::query(
                        "UPDATE videos SET
                                    compression_status = 'completed',
                                    compressed_video_path = $1,
                                    raw_video_path = NULL,
                                    updated_at = NOW()
                                WHERE id = $2",
                    )
                    .bind(zip_path.to_str().unwrap())
                    .bind(&video_id)
                    .execute(db)
                    .await?;

                    tracing::info!("Successfully compressed video {}", &video_id);
                }
                Err(err) => {
                    // Update the video record with failed status
                    sqlx::query(
                        "UPDATE videos SET
                                    compression_status = 'failed',
                                    updated_at = NOW()
                                WHERE id = $1",
                    )
                    .bind(&video_id)
                    .execute(db)
                    .await?;

                    tracing::error!("Failed to compress video {}: {}", &video_id, err);
                    return Err(err);
                }
            }
        }
        _ => tracing::warn!("Unhandled job message passed"),
    }

    Ok(())
}

/// Get the path to ffmpeg
fn get_ffmpeg_location() -> String {
    std::env::var("FFMPEG_LOCATION").unwrap_or_else(|_| "/usr/bin/ffmpeg".to_string())
}

/// Get the directory for where to store videos
fn get_storage_dir() -> String {
    let storage_dir = std::env::var("STORAGE").unwrap_or_else(|_| "storage/".to_string());
    std::fs::create_dir_all(&storage_dir).unwrap();
    storage_dir
}
