use std::path::PathBuf;

use anyhow::anyhow;
use aws_sdk_s3::Client;
use db::{DBPool, Video};

pub mod stream;

pub struct Vod {
    video: Video,
}

pub struct DownloadSettings {
    client: Client,
    bucket: String,
}

impl Vod {
    /// Gets the VOD from the database by ID
    pub async fn by_id(pool: &DBPool, id: String) -> Result<Self, anyhow::Error> {
        let video = Video::by_id(pool, &id)
            .await
            .map_err(|e| anyhow!("Could not get VOD by video ID {} {}", &id, e))?;

        Ok(Vod { video })
    }
    /// Gets the raw video locally, and optionally downloads it if missing
    pub async fn get_raw_video(
        &self,
        target_folder: PathBuf,
        download_settings: Option<DownloadSettings>,
    ) -> Result<PathBuf, anyhow::Error> {
        if !target_folder.is_dir() {
            return Err(anyhow!("Path is not a directory"));
        }
        tracing::debug!(
            "Checking {} for video {} raw files",
            target_folder.to_str().unwrap(),
            self.video.id
        );
        let file_key = if let Some(cap) = self.video.raw_video_path.split('/').last() {
            cap.to_string()
        } else {
            return Err(anyhow!("Invalid video path"));
        };

        let local_file_path = target_folder.join(&self.video.id).join(&file_key);
        if !local_file_path.exists() {
            if let Some(settings) = download_settings {
                self.download_raw(settings, &local_file_path).await?;
            } else {
                return Err(anyhow!(
                    "Video does not exist locally, skipping downloading"
                ));
            }
        }

        Ok(local_file_path)
    }
    /// Downloads the raw video from S3 to the target path
    pub async fn download_raw(
        &self,
        settings: DownloadSettings,
        target_path: &PathBuf,
    ) -> Result<(), anyhow::Error> {
        // The source video is expected to be in R2
        let folder = target_path.parent().unwrap();
        std::fs::create_dir_all(folder).map_err(|e| anyhow!("Failed to create folders: {}", e))?;

        tracing::debug!(
            "Downloading raw video from path: {}",
            &self.video.raw_video_path
        );
        let req = settings
            .client
            .get_object()
            .bucket(settings.bucket)
            .key(&self.video.raw_video_path)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to download from R2: {}", e))?;

        let bytes = req.body.collect().await?;
        std::fs::write(target_path, bytes.into_bytes())
            .map_err(|e| anyhow!("Failed to save file: {}", e))?;

        Ok(())
    }
}
