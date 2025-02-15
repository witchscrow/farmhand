-- Create enum type for privacy status
CREATE TYPE privacy_status AS ENUM ('private', 'public');

-- Add privacy_status column to videos table with default value of 'public'
ALTER TABLE videos
ADD COLUMN privacy_status privacy_status NOT NULL DEFAULT 'public';
