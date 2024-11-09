use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct HLSConverter {
    ffmpeg_path: PathBuf,
    output_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Quality {
    pub width: u32,
    pub height: u32,
    pub bitrate: String,
    pub name: String,
}

impl Quality {
    pub fn new(
        width: u32,
        height: u32,
        bitrate: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            width,
            height,
            bitrate: bitrate.into(),
            name: name.into(),
        }
    }
}

impl HLSConverter {
    pub fn new<P: AsRef<Path>>(ffmpeg_path: P, output_dir: P) -> Result<Self> {
        let ffmpeg = ffmpeg_path.as_ref().to_path_buf();
        let out_dir = output_dir.as_ref().to_path_buf();

        if !ffmpeg.exists() {
            anyhow::bail!("FFmpeg not found at {:?}", ffmpeg);
        }

        std::fs::create_dir_all(&out_dir).context("Failed to create output directory")?;

        Ok(Self {
            ffmpeg_path: ffmpeg,
            output_dir: out_dir,
        })
    }

    pub fn convert_to_hls<P: AsRef<Path>>(
        &self,
        input_path: P,
        qualities: Vec<Quality>,
    ) -> Result<()> {
        let input_path = input_path.as_ref();
        if !input_path.exists() {
            anyhow::bail!("Input file not found: {:?}", input_path);
        }

        // Create variant playlist
        let mut master_playlist = String::from("#EXTM3U\n#EXT-X-VERSION:3\n");

        // Process each quality
        for quality in qualities.iter() {
            let output_name = format!("stream_{}", quality.name);
            let playlist_name = format!("{}.m3u8", output_name);
            let segment_pattern = format!("{}_segment_%03d.ts", output_name);

            // Add to variant playlist
            master_playlist.push_str(&format!(
                "#EXT-X-STREAM-INF:BANDWIDTH={},RESOLUTION={}x{},NAME=\"{}\"\n{}\n",
                quality.bitrate.replace("k", "000"),
                quality.width,
                quality.height,
                quality.name,
                playlist_name
            ));

            // Convert for this quality
            self.convert_quality(
                input_path,
                &output_name,
                quality,
                &playlist_name,
                &segment_pattern,
            )
            .with_context(|| format!("Failed to convert quality: {}", quality.name))?;
        }

        // Write master playlist
        std::fs::write(self.output_dir.join("master.m3u8"), master_playlist)
            .context("Failed to write master playlist")?;

        Ok(())
    }

    fn convert_quality(
        &self,
        input_path: &Path,
        output_name: &str,
        quality: &Quality,
        playlist_name: &str,
        segment_pattern: &str,
    ) -> Result<()> {
        let output = Command::new(&self.ffmpeg_path)
            .arg("-i")
            .arg(input_path)
            .arg("-c:v")
            .arg("libx264")
            .arg("-c:a")
            .arg("aac")
            .arg("-b:v")
            .arg(&quality.bitrate)
            .arg("-maxrate")
            .arg(&quality.bitrate)
            .arg("-bufsize")
            .arg(format!(
                "{}k",
                quality.bitrate.replace("k", "").parse::<u32>()? * 2
            ))
            .arg("-preset")
            .arg("faster")
            .arg("-g")
            .arg("60")
            .arg("-sc_threshold")
            .arg("0")
            .arg("-s")
            .arg(format!("{}x{}", quality.width, quality.height))
            .arg("-f")
            .arg("hls")
            .arg("-hls_time")
            .arg("6")
            .arg("-hls_list_size")
            .arg("0")
            .arg("-hls_segment_type")
            .arg("mpegts")
            .arg("-hls_segment_filename")
            .arg(self.output_dir.join(segment_pattern))
            .arg(self.output_dir.join(playlist_name))
            .output()
            .context("Failed to execute FFmpeg command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("FFmpeg failed: {}", error);
        }

        Ok(())
    }

    pub fn verify_ffmpeg(&self) -> Result<String> {
        let output = Command::new(&self.ffmpeg_path)
            .arg("-version")
            .output()
            .context("Failed to execute FFmpeg version command")?;

        if !output.status.success() {
            anyhow::bail!("FFmpeg version check failed");
        }

        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .unwrap_or("Unknown version")
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hls_conversion() -> Result<()> {
        let converter = HLSConverter::new("/usr/bin/ffmpeg", "output")?;

        println!("FFmpeg version: {}", converter.verify_ffmpeg()?);

        let qualities = vec![
            Quality::new(1920, 1080, "5000k", "1080p"),
            Quality::new(1280, 720, "2800k", "720p"),
            Quality::new(854, 480, "1400k", "480p"),
            Quality::new(640, 360, "800k", "360p"),
        ];

        converter.convert_to_hls("input.mp4", qualities)?;
        Ok(())
    }
}
