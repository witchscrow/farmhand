CREATE TYPE processing_status AS ENUM ('pending', 'processing', 'completed', 'failed');

CREATE TABLE videos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    raw_video_path VARCHAR(255) NOT NULL,
    processed_video_path VARCHAR(255),
    processing_status processing_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX videos_user_id_idx ON videos(user_id);
