use crate::error::Error;
use crate::{Job, Message, Queue};
use db::Video;
use futures::{stream, StreamExt};
use sqlx::{Pool, Postgres};
use std::{path::PathBuf, sync::Arc, time::Duration};
use vod::Vod;

/// Runs a loop that pulls jobs from the queue and runs <concurrency> jobs each loop
pub async fn run_worker(queue: Arc<dyn Queue>, concurrency: usize, db_conn: &Pool<Postgres>) {
    loop {
        // Pulls jobs from the queue
        let jobs = match queue.pull(concurrency as i32).await {
            Ok(jobs) => jobs,
            Err(err) => {
                // Trace the error
                tracing::error!("runner: error pulling jobs {}", err);
                // Go to sleep and try again
                tokio::time::sleep(Duration::from_millis(500)).await;
                Vec::new()
            }
        };
        // Just for debugging the amount of jobs a queue has pulled in
        let number_of_jobs = jobs.len();
        if number_of_jobs > 0 {
            tracing::debug!("Fetched {} jobs", number_of_jobs);
        }
        // Run each jobs concurrently
        stream::iter(jobs)
            .for_each_concurrent(concurrency, |job| async {
                tracing::debug!("Starting job {}", job.id);
                let job_id = job.id;

                let res = match handle_job(job, db_conn).await {
                    Ok(_) => queue.delete_job(job_id).await,
                    Err(err) => {
                        println!("run_worker: handling job({}): {}", job_id, &err);
                        queue.fail_job(job_id).await
                    }
                };

                match res {
                    Ok(_) => {}
                    Err(err) => {
                        println!("run_worker: deleting / failing job: {}", &err);
                    }
                }
            })
            .await;
        // Take a break for a bit, we don't need to run every moment (our jobs are unlikely to complete that quickly)
        tokio::time::sleep(Duration::from_millis(125)).await;
    }
}

/// Individually processes a single job, based on its Job message type
async fn handle_job(job: Job, db: &Pool<Postgres>) -> Result<(), Error> {
    match job.message {
        Message::ProcessRawVideoIntoStream { video_id } => {
            // First, update video status to Processing
            sqlx::query(
                "UPDATE videos SET processing_status = 'processing', updated_at = NOW() WHERE id = $1"
            )
            .bind(video_id)
            .execute(db)
            .await?;

            // Get video details from database
            let video = sqlx::query_as::<_, Video>("SELECT * FROM videos WHERE id = $1")
                .bind(video_id)
                .fetch_one(db)
                .await?;

            // Create output directory path
            let output_dir = PathBuf::from("processed_videos").join(video_id.to_string());

            // Create VOD instance
            let vod = Vod::new(PathBuf::from(&video.raw_video_path));

            // Process the video
            let result = vod.convert_video_to_stream(&output_dir);

            match result {
                Ok(master_playlist_path) => {
                    // Update video with success status and processed path
                    sqlx::query(
                        "UPDATE videos SET
                            processing_status = 'completed',
                            processed_video_path = $1,
                            updated_at = NOW()
                        WHERE id = $2",
                    )
                    .bind(master_playlist_path.to_str().unwrap())
                    .bind(video_id)
                    .execute(db)
                    .await?;

                    tracing::info!("Successfully processed video {}", video_id);
                }
                Err(e) => {
                    // Update video with failed status
                    sqlx::query(
                        "UPDATE videos SET
                            processing_status = 'failed',
                            updated_at = NOW()
                        WHERE id = $1",
                    )
                    .bind(video_id)
                    .execute(db)
                    .await?;

                    tracing::error!("Failed to process video {}: {}", video_id, e);
                    return Err(Error::VideoProcessingError(e.to_string()));
                }
            }
        }
    }

    Ok(())
}
