set dotenv-load := true

# Service startup commands
start-api:
    cargo run -p api

start-web:
    yarn start

start-queue:
    cargo run -p queue

# Development commands
dev-api: start-api

dev-web:
    yarn dev

dev-queue: start-queue

# Database commands
init-db: create-db migrate

create-db:
    sqlx database create

drop-db:
    sqlx database drop

mig_source := "packages/db/migrations"

mig: migrate
migrate:
    sqlx migrate run --source {{ mig_source }}

mig-add mig_name:
    sqlx migrate add {{ mig_name }} --source {{ mig_source }}

revert:
    sqlx migrate run --source {{ mig_source }}

# Build commands
build-web:
    yarn build

build-api:
    cargo build -p api --release

build-queue:
    cargo build -p forge --release

# Docker build commands - Queue Service (forge-queue)
biq: build-image-queue  # Shorthand for building queue image
build-image-queue: (build-image-queue-local)  # Default to local build for queue

# Build queue image for local development (M1/ARM64)
build-image-queue-local:
    docker build --platform linux/arm64 -t forge-queue -f config/queue.Dockerfile .

# Build queue image for production (AMD64)
build-image-queue-prod:
    docker build --platform linux/amd64 -t forge-queue -f config/queue.Dockerfile .

# Run queue container with local build
run-queue: build-image-queue
    docker run -e DATABASE_URL=${DATABASE_URL} forge-queue

# Run queue container with production build
run-queue-prod: build-image-queue-prod
    docker run -e DATABASE_URL=${DATABASE_URL} forge-queue

# Docker build commands - API Service (silo-api)
bia: build-image-api  # Shorthand for building API image
build-image-api: (build-image-api-local)  # Default to local build for API

# Build API image for local development (M1/ARM64)
build-image-api-local:
    docker build --platform linux/arm64 -t silo-api -f config/api.Dockerfile .

# Build API image for production (AMD64)
build-image-api-prod:
    docker build --platform linux/amd64 -t silo-api -f config/api.Dockerfile .

# Run API container with local build
run-api: build-image-api
    docker run -p 3000:3000 -e DATABASE_URL=${DATABASE_URL} -e JWT_SECRET=${JWT_SECRET} silo-api

# Run API container with production build
run-api-prod: build-image-api-prod
    docker run -p 3000:3000 -e DATABASE_URL=${DATABASE_URL} -e JWT_SECRET=${JWT_SECRET} silo-api

# Docker build commands - UI Service (barn-ui)
biu: build-image-ui  # Shorthand for building UI image
build-image-ui: (build-image-ui-local)  # Default to local build for UI

# Build UI image for local development (M1/ARM64)
build-image-ui-local:
    docker build --platform linux/arm64 -t barn-ui -f config/ui.Dockerfile .

# Build UI image for production (AMD64)
build-image-ui-prod:
    docker build --platform linux/amd64 -t barn-ui -f config/ui.Dockerfile .

# Run UI container with local build
run-ui: build-image-ui
    docker run -p 3000:3000 barn-ui

# Run UI container with production build
run-ui-prod: build-image-ui-prod
    docker run -p 3000:3000 barn-ui

# Utility commands
sync: sync-web
sync-web:
    yarn sync

# Clean commands
clean:
    cargo clean
    rm -rf node_modules
    docker rmi forge-queue || true

# Verification commands
verify:
    cargo check
    cargo test
    cargo clippy -- -D warnings
    cargo fmt --all -- --check

# Run all services
dev: dev-web dev-api dev-queue

# Build all services
build: build-web build-api build-queue
