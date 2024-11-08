# forge

Forge is Farmhand's dedicated video processing queue service. It transforms raw uploads into optimized content through various encoding and analysis processes.

## Technical Overview
- **Runtime**: Rust
- **Job Queue**: Custom implementation with PostgreSQL
- **Processing**: FFmpeg integration via vod crate
- **Error Handling**: anyhow/thiserror based robust error management

## Features
- 🎬 Video transcoding queue management
- 🔄 Concurrent job processing with 3 workers
- 🎯 Job prioritization and scheduling
- 📊 Processing status tracking in database
- 🔍 Automatic retries (up to 5 attempts)
- 🎥 HLS stream output format

## Getting Started

First, set up your environment variables:

```bash
# Required
DATABASE_URL=postgres://user:password@localhost:5432/dbname

# Optional
RUST_LOG=debug  # Logging level
```

Then run the service:

```bash
cargo run
```

## Jobs

Currently processes:
- `ProcessRawVideoIntoStream`: Converts raw video files into HLS format
  - Updates processing status in real-time
  - Creates segmented video streams
  - Generates master playlist

## Architecture

The queue service consists of:
- PostgreSQL-backed persistent job store
- Concurrent worker pool (3 workers)
- Automatic job status management
- Failed job recovery system

Jobs flow through three states:
- `Queued`: Awaiting processing
- `Running`: Under active conversion
- `Failed`: Error occurred (automatic retry)

## Development

Service organization:
```
src/
  ├── main.rs      # Worker service entry
  ├── queue.rs     # Queue implementation
  ├── job.rs       # Job definitions
  ├── runner.rs    # Processing logic
  └── error.rs     # Error handling
```
