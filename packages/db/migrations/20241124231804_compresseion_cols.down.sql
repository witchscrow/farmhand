-- Remove compression-related columns
ALTER TABLE videos
    DROP COLUMN compression_status,
    DROP COLUMN compressed_video_path;

-- Drop the compression status enum type
DROP TYPE compression_status;
