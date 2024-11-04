use anyhow::Result;
use ffmpeg_next as ffmpeg;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Defines the characteristics for each video quality level
#[derive(Debug, Clone)]
pub struct VideoQuality {
    pub name: String, // Quality name (e.g., "1080p")
    pub height: u32,  // Target height in pixels
    pub bitrate: u64, // Target bitrate in bits per second
}

/// The different formats of video file we support
#[derive(Debug)]
pub enum RawVideoType {
    MP4,
}

/// A representation of a raw video, including type and location
#[derive(Debug)]
pub struct RawVideo {
    video_type: RawVideoType,
    path: PathBuf,
}

/// Settings for HLS stream conversion
#[derive(Debug)]
pub struct StreamSettings {
    pub qualities: Vec<VideoQuality>,
    pub segment_duration: u32, // Duration of each segment in seconds
    pub keep_all_segments: bool,
}

impl Default for StreamSettings {
    fn default() -> Self {
        Self {
            qualities: vec![
                VideoQuality {
                    name: "1080p".to_string(),
                    height: 1080,
                    bitrate: 5_000_000, // 5 Mbps
                },
                VideoQuality {
                    name: "720p".to_string(),
                    height: 720,
                    bitrate: 2_500_000, // 2.5 Mbps
                },
                VideoQuality {
                    name: "480p".to_string(),
                    height: 480,
                    bitrate: 1_000_000, // 1 Mbps
                },
            ],
            segment_duration: 10,
            keep_all_segments: true,
        }
    }
}

/// A vod (video on demand) represents a raw video, stream, and relative associations
#[derive(Debug)]
pub struct Vod {
    raw_video: RawVideo,
    stream_settings: StreamSettings,
}

impl Vod {
    /// Creates a new VOD instance with default stream settings
    pub fn new(video_path: PathBuf) -> Self {
        Self {
            raw_video: RawVideo {
                video_type: RawVideoType::MP4,
                path: video_path,
            },
            stream_settings: StreamSettings::default(),
        }
    }

    /// Creates a new VOD instance with custom stream settings
    pub fn new_with_settings(video_path: PathBuf, settings: StreamSettings) -> Self {
        Self {
            raw_video: RawVideo {
                video_type: RawVideoType::MP4,
                path: video_path,
            },
            stream_settings: settings,
        }
    }

    /// Returns the current stream settings
    pub fn stream_settings(&self) -> &StreamSettings {
        &self.stream_settings
    }

    /// Updates the stream settings
    pub fn set_stream_settings(&mut self, settings: StreamSettings) {
        self.stream_settings = settings;
    }

    /// Converts a raw video into multiple HLS streams of different qualities
    /// Returns the path to the master playlist file
    pub fn convert_video_to_stream(&self, output_dir: &Path) -> Result<PathBuf> {
        // Initialize FFmpeg
        ffmpeg::init()?;

        // Create output directory if it doesn't exist
        std::fs::create_dir_all(output_dir)?;

        // Open input context
        let input_ctx = ffmpeg::format::input(&self.raw_video.path)?;

        // Find the best video and audio streams from input
        let input_video_stream = input_ctx
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or_else(|| anyhow::anyhow!("No video stream found"))?;

        let input_audio_stream = input_ctx
            .streams()
            .best(ffmpeg::media::Type::Audio)
            .ok_or_else(|| anyhow::anyhow!("No audio stream found"))?;

        // Create master playlist
        let master_playlist_path = output_dir.join("master.m3u8");
        let mut master_playlist = std::fs::File::create(&master_playlist_path)?;
        writeln!(master_playlist, "#EXTM3U")?;
        writeln!(master_playlist, "#EXT-X-VERSION:3")?;

        // Store output contexts for each quality
        let mut output_contexts = Vec::new();

        // Create output streams for each quality level
        for quality in &self.stream_settings.qualities {
            // Create directory for this quality level
            let quality_dir = output_dir.join(&quality.name);
            std::fs::create_dir_all(&quality_dir)?;

            // Setup output context for this quality
            let playlist_path = quality_dir.join("stream.m3u8");
            let mut output_ctx = ffmpeg::format::output(&playlist_path)?;

            // Configure HLS output format
            output_ctx.set_format("hls");
            let mut dict = ffmpeg::Dictionary::new();
            dict.set(
                "hls_time",
                &self.stream_settings.segment_duration.to_string(),
            );
            dict.set(
                "hls_list_size",
                if self.stream_settings.keep_all_segments {
                    "0"
                } else {
                    "5"
                },
            );
            dict.set("hls_flags", "independent_segments");
            dict.set(
                "hls_segment_filename",
                quality_dir.join("segment_%03d.ts").to_str().unwrap(),
            );
            output_ctx.set_options(dict);

            // Setup video encoder for this quality
            let encoder_video = ffmpeg::encoder::find(ffmpeg::codec::Id::H264)
                .ok_or_else(|| anyhow::anyhow!("H264 encoder not found"))?;
            let mut video_stream = output_ctx.add_stream(encoder_video)?;
            let mut video_encoder = video_stream.codec().encoder().video()?;

            // Calculate width maintaining aspect ratio
            let input_width = input_video_stream.codec().width();
            let input_height = input_video_stream.codec().height();
            let width = (input_width as f32 * (quality.height as f32 / input_height as f32)) as u32;
            let width = width - (width % 2); // Ensure even width for H.264

            // Configure video encoder parameters
            video_encoder.set_width(width);
            video_encoder.set_height(quality.height);
            video_encoder.set_format(ffmpeg::format::Pixel::YUV420P);
            video_encoder.set_bit_rate(quality.bitrate);
            video_encoder.set_max_b_frames(2);
            video_encoder.set_time_base((1, 30)); // 30 fps

            // Additional H.264 specific settings
            let mut codec_opts = ffmpeg::Dictionary::new();
            codec_opts.set("preset", "medium");
            codec_opts.set("profile", "high");
            codec_opts.set("level", "4.1");
            video_encoder.set_options(codec_opts);

            video_encoder.open_as(encoder_video)?;

            // Add this quality variant to master playlist
            writeln!(
                master_playlist,
                "#EXT-X-STREAM-INF:BANDWIDTH={},RESOLUTION={}x{}",
                quality.bitrate, width, quality.height
            )?;
            writeln!(master_playlist, "{}/stream.m3u8", quality.name)?;

            // Setup audio encoder (same settings for all qualities)
            let encoder_audio = ffmpeg::encoder::find(ffmpeg::codec::Id::AAC)
                .ok_or_else(|| anyhow::anyhow!("AAC encoder not found"))?;
            let mut audio_stream = output_ctx.add_stream(encoder_audio)?;
            let mut audio_encoder = audio_stream.codec().encoder().audio()?;

            // Configure audio encoder parameters
            audio_encoder.set_rate(44100);
            audio_encoder.set_channels(2);
            audio_encoder.set_channel_layout(ffmpeg::channel_layout::ChannelLayout::STEREO);
            audio_encoder.set_format(ffmpeg::format::Sample::F32(
                ffmpeg::format::sample::Type::Packed,
            ));
            audio_encoder.set_bit_rate(128_000); // 128 kbps audio

            audio_encoder.open_as(encoder_audio)?;

            // Write the header for this quality's stream
            output_ctx.write_header()?;

            output_contexts.push((output_ctx, video_encoder, audio_encoder));
        }

        // Setup decoders for input streams
        let mut video_decoder = input_video_stream.codec().decoder().video()?;
        let mut audio_decoder = input_audio_stream.codec().decoder().audio()?;

        // Process all input packets
        for (stream, packet) in input_ctx.packets() {
            match stream.index() {
                // Handle video packets
                i if i == input_video_stream.index() => {
                    let mut decoded = ffmpeg::frame::Video::empty();
                    if video_decoder.decode(&packet, &mut decoded)? {
                        // Process each quality level
                        for ((output_ctx, video_encoder, _), quality) in output_contexts
                            .iter_mut()
                            .zip(self.stream_settings.qualities.iter())
                        {
                            let mut encoded = ffmpeg::Packet::empty();
                            if video_encoder.encode(&decoded, &mut encoded)? {
                                encoded.set_stream(0);
                                output_ctx.write_frame(&encoded)?;
                            }
                        }
                    }
                }
                // Handle audio packets
                i if i == input_audio_stream.index() => {
                    let mut decoded = ffmpeg::frame::Audio::empty();
                    if audio_decoder.decode(&packet, &mut decoded)? {
                        // Process audio for each quality (same audio for all qualities)
                        for (output_ctx, _, audio_encoder) in output_contexts.iter_mut() {
                            let mut encoded = ffmpeg::Packet::empty();
                            if audio_encoder.encode(&decoded, &mut encoded)? {
                                encoded.set_stream(1);
                                output_ctx.write_frame(&encoded)?;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Flush encoders and write trailers
        for (output_ctx, video_encoder, audio_encoder) in output_contexts {
            let mut encoded = ffmpeg::Packet::empty();

            // Flush video encoder
            while video_encoder.flush(&mut encoded)? {
                encoded.set_stream(0);
                output_ctx.write_frame(&encoded)?;
            }

            // Flush audio encoder
            while audio_encoder.flush(&mut encoded)? {
                encoded.set_stream(1);
                output_ctx.write_frame(&encoded)?;
            }

            output_ctx.write_trailer()?;
        }

        Ok(master_playlist_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_qualities() {
        // Create custom settings
        let settings = StreamSettings {
            qualities: vec![
                VideoQuality {
                    name: "4K".to_string(),
                    height: 2160,
                    bitrate: 20_000_000,
                },
                VideoQuality {
                    name: "1080p".to_string(),
                    height: 1080,
                    bitrate: 5_000_000,
                },
            ],
            segment_duration: 6,
            keep_all_segments: false,
        };

        // Create VOD with custom settings
        let vod = Vod::new_with_settings(PathBuf::from("input.mp4"), settings);

        assert_eq!(vod.stream_settings().qualities.len(), 2);
        assert_eq!(vod.stream_settings().segment_duration, 6);
        assert_eq!(vod.stream_settings().keep_all_segments, false);
    }
}
