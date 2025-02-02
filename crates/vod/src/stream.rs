use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, warn};

#[derive(Debug, Clone, PartialEq)]
pub enum VideoFormat {
    MP4,
    MOV,
}

impl VideoFormat {
    fn from_path(path: &Path) -> Result<Self> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
            .ok_or_else(|| anyhow::anyhow!("Input file has no extension"))?;

        match extension.as_str() {
            "mp4" => Ok(VideoFormat::MP4),
            "mov" => Ok(VideoFormat::MOV),
            _ => anyhow::bail!("Unsupported file format: {}", extension),
        }
    }

    fn get_ffmpeg_args(&self) -> Vec<String> {
        match self {
            VideoFormat::MP4 => vec!["-movflags".to_string(), "+faststart".to_string()],
            VideoFormat::MOV => vec![
                "-movflags".to_string(),
                "+faststart".to_string(),
                "-strict".to_string(),
                "experimental".to_string(),
            ],
        }
    }

    fn get_hls_args(&self) -> Vec<String> {
        vec![
            "-f".to_string(),
            "hls".to_string(),
            "-hls_time".to_string(),
            "6".to_string(),
            "-hls_list_size".to_string(),
            "0".to_string(),
            "-hls_segment_type".to_string(),
            "mpegts".to_string(),
            "-hls_flags".to_string(),
            "independent_segments+split_by_time".to_string(),
        ]
    }
}

#[derive(Clone)]
pub struct HLSConverter {
    pub ffmpeg_path: PathBuf,
    pub output_dir: PathBuf,
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
    fn get_video_dimensions(&self, input_path: &Path) -> Result<(u32, u32)> {
        debug!("Getting dimensions for {:?}", input_path);
        let output = Command::new(&self.ffmpeg_path)
            .arg("-i")
            .arg(input_path)
            .output()
            .context("Failed to execute FFmpeg command for video info")?;

        let stderr = String::from_utf8_lossy(&output.stderr);
        debug!("FFmpeg output:\n{}", stderr);

        // Parse the video dimensions from FFmpeg output
        for line in stderr.lines() {
            if line.contains("Stream") && line.contains("Video:") {
                debug!("Found video stream line: {}", line);

                // Try different patterns
                let dimensions = line
                    .split(',')
                    .find(|s| s.contains('x') && s.trim().chars().any(|c| c.is_digit(10)))
                    .or_else(|| {
                        // Alternative pattern: look for dimensions like "1920x1080"
                        line.split_whitespace()
                            .find(|s| s.contains('x') && s.chars().any(|c| c.is_digit(10)))
                    });

                if let Some(dim_str) = dimensions {
                    debug!("Found dimension string: {}", dim_str);

                    // Clean up the dimension string
                    let clean_dim = dim_str
                        .trim()
                        .split(|c: char| !c.is_digit(10) && c != 'x')
                        .collect::<String>();

                    if let Some(x_pos) = clean_dim.find('x') {
                        if let (Ok(width), Ok(height)) = (
                            clean_dim[..x_pos].parse::<u32>(),
                            clean_dim[x_pos + 1..].parse::<u32>(),
                        ) {
                            debug!("Parsed dimensions: {}x{}", width, height);
                            return Ok((width, height));
                        }
                    }
                }
            }
        }

        // If the above fails, try using ffprobe
        let probe_output = Command::new(&self.ffmpeg_path.with_file_name("ffprobe"))
            .arg("-v")
            .arg("error")
            .arg("-select_streams")
            .arg("v:0")
            .arg("-show_entries")
            .arg("stream=width,height")
            .arg("-of")
            .arg("csv=p=0")
            .arg(input_path)
            .output()
            .context("Failed to execute ffprobe command")?;

        if probe_output.status.success() {
            let output = String::from_utf8_lossy(&probe_output.stdout);
            let dims: Vec<&str> = output.trim().split(',').collect();
            if dims.len() == 2 {
                if let (Ok(width), Ok(height)) = (dims[0].parse::<u32>(), dims[1].parse::<u32>()) {
                    debug!("Got dimensions from ffprobe: {}x{}", width, height);
                    return Ok((width, height));
                }
            }
        }

        anyhow::bail!("Could not determine video dimensions")
    }

    fn verify_dimensions(&self, width: u32, height: u32) -> Result<()> {
        if width == 0 || height == 0 {
            anyhow::bail!("Invalid dimensions: {}x{}", width, height);
        }
        if width > 7680 || height > 4320 {
            // 8K limit
            anyhow::bail!("Dimensions too large: {}x{}", width, height);
        }
        Ok(())
    }

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

    fn validate_input_format(&self, input_path: &Path) -> Result<VideoFormat> {
        VideoFormat::from_path(input_path)
    }

    pub fn convert_to_hls<P: AsRef<Path>>(
        &self,
        input_path: P,
        mut qualities: Vec<Quality>,
    ) -> Result<()> {
        let input_path = input_path.as_ref();
        if !input_path.exists() {
            anyhow::bail!("Input file not found: {:?}", input_path);
        }

        let format = self.validate_input_format(input_path)?;

        // Get original video dimensions
        let (original_width, original_height) = self.get_video_dimensions(input_path)?;
        self.verify_dimensions(original_width, original_height)?;

        // Filter out qualities higher than the original resolution
        qualities.retain(|q| {
            if q.width > original_width || q.height > original_height {
                warn!(
                    "Skipping quality {}x{} as it exceeds original resolution {}x{}",
                    q.width, q.height, original_width, original_height
                );
                false
            } else {
                true
            }
        });

        if qualities.is_empty() {
            anyhow::bail!(
                "No valid quality levels for video with resolution {}x{}",
                original_width,
                original_height
            );
        }

        // Create variant playlist
        let mut master_playlist = String::from("#EXTM3U\n#EXT-X-VERSION:3\n");

        // Process each quality
        for quality in qualities.iter() {
            let output_name = format!("stream_{}", quality.name);
            let playlist_name = format!("{}.m3u8", output_name);
            let segment_pattern = format!("{}_segment_%03d.ts", output_name);

            // Add to variant playlist with updated path that includes quality directory
            master_playlist.push_str(&format!(
                "#EXT-X-STREAM-INF:BANDWIDTH={},RESOLUTION={}x{},NAME=\"{}\"\n{}/{}\n",
                quality.bitrate.replace("k", "000"),
                quality.width,
                quality.height,
                quality.name,
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
                &format,
            )
            .with_context(|| {
                format!(
                    "Failed to convert quality: {} ({}x{})",
                    quality.name, quality.width, quality.height
                )
            })?;
        }

        // Write master playlist in the root output directory
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
        format: &VideoFormat,
    ) -> Result<()> {
        // Create quality-specific directory
        let quality_dir = self.output_dir.join(&quality.name);
        std::fs::create_dir_all(&quality_dir)
            .context("Failed to create quality-specific directory")?;

        let mut command = Command::new(&self.ffmpeg_path);

        command.arg("-i").arg(input_path);

        // Add format-specific arguments
        for arg in format.get_ffmpeg_args() {
            command.arg(arg);
        }

        command
            .arg("-vsync")
            .arg("0")
            // Video encoding settings
            .arg("-c:v")
            .arg("libx264")
            .arg("-c:a")
            .arg("aac")
            // Force pixel format
            .arg("-pix_fmt")
            .arg("yuv420p")
            // Bitrate settings
            .arg("-b:v")
            .arg(&quality.bitrate)
            .arg("-maxrate")
            .arg(&quality.bitrate)
            .arg("-bufsize")
            .arg(format!(
                "{}k",
                quality.bitrate.replace("k", "").parse::<u32>()? * 2
            ))
            // Encoding presets
            .arg("-preset")
            .arg("faster")
            .arg("-profile:v")
            .arg("main")
            .arg("-level")
            .arg("3.1")
            .arg("-g")
            .arg("60")
            .arg("-keyint_min")
            .arg("60")
            .arg("-sc_threshold")
            .arg("0")
            .arg("-force_key_frames")
            .arg("expr:gte(t,n_forced*6)")
            // Resolution
            .arg("-s")
            .arg(format!("{}x{}", quality.width, quality.height))
            // Audio settings
            .arg("-ar")
            .arg("48000")
            .arg("-ac")
            .arg("2")
            .arg("-b:a")
            .arg("128k");

        // Add HLS-specific settings
        for arg in format.get_hls_args() {
            command.arg(arg);
        }

        command
            .arg("-hls_segment_filename")
            .arg(quality_dir.join(segment_pattern))
            .arg(quality_dir.join(playlist_name));

        debug!("FFmpeg command: {:?}", command);

        let output = command
            .output()
            .context("Failed to execute FFmpeg command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            debug!("FFmpeg error output: {}", error);
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

/// Get the path to ffmpeg
pub fn get_ffmpeg_location() -> PathBuf {
    let env_ffmpeg_path = PathBuf::from(
        std::env::var("FFMPEG_LOCATION").unwrap_or_else(|_| "/usr/bin/ffmpeg".to_string()),
    );

    if !env_ffmpeg_path.exists() {
        panic!("FFmpeg not found at path: {:?}", env_ffmpeg_path);
    }

    env_ffmpeg_path
}
