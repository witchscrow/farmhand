use crate::error::Error;
use crate::queue::{Job, Message, Queue};
use db::Video;
use futures::{stream, StreamExt};
use sqlx::{Pool, Postgres};
use std::{path::PathBuf, sync::Arc, time::Duration};
use vod::{HLSConverter, Quality};

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

                let res = match handle_job(job, db_conn).await {
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
async fn handle_job(job: Job, db: &Pool<Postgres>) -> Result<(), Error> {
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

            // Get video details
            let video = sqlx::query_as::<_, Video>("SELECT * FROM videos WHERE id = $1")
                .bind(&video_id)
                .fetch_one(db)
                .await?;

            // Create output directory
            let output_dir = PathBuf::from(get_videos_dir()).join(&video_id.to_string());
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
            match converter.convert_to_hls(&video.raw_video_path, qualities) {
                Ok(_) => {
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
                    .bind(&video_id)
                    .execute(db)
                    .await?;

                    tracing::info!("Successfully processed video {}", &video_id);
                }
                Err(e) => {
                    // Update with failed status
                    sqlx::query(
                        "UPDATE videos SET
                            processing_status = 'failed',
                            updated_at = NOW()
                        WHERE id = $1",
                    )
                    .bind(&video_id)
                    .execute(db)
                    .await?;

                    tracing::error!("Failed to process video {}: {}", &video_id, e);
                    return Err(Error::VideoProcessingError(e.to_string()));
                }
            }
        }
    }

    Ok(())
}

/// Get the path to ffmpeg
fn get_ffmpeg_location() -> String {
    std::env::var("FFMPEG_LOCATION").unwrap_or_else(|_| "/usr/bin/ffmpeg".to_string())
}

/// Get the directory for where to store videos
fn get_videos_dir() -> String {
    std::env::var("VIDEOS_DIR").unwrap_or_else(|_| "videos".to_string())
}
