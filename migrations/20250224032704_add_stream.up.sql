CREATE TABLE IF NOT EXISTS streams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    start_time TIMESTAMP
    WITH
        TIME ZONE NOT NULL,
        end_time TIMESTAMP
    WITH
        TIME ZONE,
        event_log_url TEXT,
        video_url TEXT,
        created_at TIMESTAMP
    WITH
        TIME ZONE NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP
    WITH
        TIME ZONE NOT NULL DEFAULT NOW ()
);

-- Add an index on start_time for efficient querying
CREATE INDEX idx_streams_start_time ON streams (start_time);

-- Create trigger using existing function
CREATE TRIGGER update_streams_updated_at BEFORE
UPDATE ON streams FOR EACH ROW EXECUTE FUNCTION update_updated_at_column ();
