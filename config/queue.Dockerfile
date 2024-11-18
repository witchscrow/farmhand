# Build stage
FROM --platform=$BUILDPLATFORM rust:1.82-slim-bullseye as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /app

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./

# Copy all workspace members
COPY packages/db ./packages/db
COPY packages/vod ./packages/vod
COPY packages/queue ./packages/queue
COPY services/forge-queue ./services/forge-queue
COPY services/silo-api ./services/silo-api

# Build the project for release
WORKDIR /app/services/forge-queue
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release -p forge && \
    cp /app/target/release/queue /usr/local/bin/queue

# Runtime stage
FROM --platform=$TARGETPLATFORM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    ffmpeg \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/local/bin/queue /usr/local/bin/queue

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/queue"]
