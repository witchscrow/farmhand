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
COPY packages/ ./packages/
COPY services/forge-queue ./services/forge-queue
COPY services/silo-api ./services/silo-api

# Build the project for release
WORKDIR /app/services/forge-queue
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release -p forge && \
    cp /app/target/release/forge /usr/local/bin/forge


# Runtime stage
FROM jrottenberg/ffmpeg:7.1-ubuntu2404-edge

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/local/bin/forge /usr/local/bin/forge

# Expose the health check server
EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/forge"]
