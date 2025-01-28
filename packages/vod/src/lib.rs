use std::path::PathBuf;

use anyhow::anyhow;
use aws_sdk_s3::Client;
use db::{DBPool, Video};
use stream::{get_ffmpeg_location, HLSConverter};

pub mod stream;

#[derive(Clone)]
pub struct Vod {
    pub video: Video,
    pub converter: HLSConverter,
}

pub struct DownloadSettings<'a> {
    pub client: &'a Client,
    pub bucket: &'a str,
}

impl Vod {
    /// Gets the VOD from the database by ID
    pub async fn by_id(
        pool: &DBPool,
        id: String,
        output_dir: PathBuf,
    ) -> Result<Self, anyhow::Error> {
        let video = Video::by_id(pool, &id)
            .await
            .map_err(|e| anyhow!("Could not get VOD by video ID {} {}", &id, e))?;
        let converter = HLSConverter::new(get_ffmpeg_location(), output_dir)
            .expect("Could not initialize HLS converter");

        Ok(Vod { video, converter })
    }
    /// Gets the prefix for the relative vod in storage
    pub fn get_remote_storage_prefix(&self) -> String {
        let storage_root = common::get_storage_dir();
        format!("{}/{}", storage_root, self.video.id)
    }
    /// Gets the raw video locally, and optionally downloads it if missing
    pub async fn get_raw_video<'a>(
        &self,
        target_folder: PathBuf,
        download_settings: Option<DownloadSettings<'a>>,
    ) -> Result<Option<PathBuf>, anyhow::Error> {
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
                return Ok(None);
            }
        }

        Ok(Some(local_file_path))
    }
    /// Downloads the raw video from S3 to the target path
    pub async fn download_raw<'a>(
        &self,
        settings: DownloadSettings<'a>,
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
