use anyhow::Result;
use serde::Deserialize;

use super::Runner;

#[derive(Deserialize)]
pub struct VideoToStreamPayload {
    pub video_id: String,
}

pub struct HlsStreamRunner;

impl Runner for HlsStreamRunner {
    type Payload = VideoToStreamPayload;

    /// Converts a raw video file to an HLS stream
    async fn process_job(&self, payload: Self::Payload) -> Result<()> {
        tracing::debug!(
            "Processing job with runner HlsStreamRunner for video ID {video_id}",
            video_id = payload.video_id,
        );

        // TODO: Implement video to HLS stream conversion
        tracing::warn!("HLS stream conversion not implemented");
        Ok(())
    }
}
