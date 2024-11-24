-- Create compression status enum type
CREATE TYPE compression_status AS ENUM ('pending', 'compressing', 'completed', 'failed');

-- Add compression-related columns
ALTER TABLE videos
    ADD COLUMN compression_status compression_status NOT NULL DEFAULT 'pending',
    ADD COLUMN compressed_video_path VARCHAR(255);
