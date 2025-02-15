-- Remove the privacy_status column from videos table
ALTER TABLE videos
DROP COLUMN privacy_status;

-- Remove the privacy_status enum type
DROP TYPE privacy_status;
