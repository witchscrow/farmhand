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
COPY crates/ ./crates/
COPY services/forge-queue ./services/forge-queue
COPY services/silo-api ./services/silo-api

# Build the project for release
WORKDIR /app
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release -p api && \
    cp /app/target/release/api /usr/local/bin/api

# Runtime stage
FROM --platform=$TARGETPLATFORM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create uploads directory
RUN mkdir -p /app/uploads && \
    chmod 777 /app/uploads

# Set working directory
WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/local/bin/api /usr/local/bin/api

# Expose the default port
EXPOSE 3000

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/api"]
