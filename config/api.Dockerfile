FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src/ src/
COPY migrations/ migrations/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src/ src/
COPY migrations/ migrations/
RUN cargo build --release --bin api

# Runtime stage
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api /usr/local/bin/api
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/api"]
