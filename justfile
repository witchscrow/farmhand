set dotenv-load := true

# Build Commands
build-api:
    cargo build --bin api

# Dev Commands
dev-ui: dev-web
dev-web:
    yarn dev

# Database commands
init-db: create-db migrate

create-db:
    sqlx database create

drop-db:
    sqlx database drop

mig_source := "crates/common/migrations"

mig: migrate
migrate:
    sqlx migrate run --source {{ mig_source }}

mig-add mig_name:
    sqlx migrate add {{ mig_name }} --source {{ mig_source }}

revert:
    sqlx migrate run --source {{ mig_source }}

# Utility commands
sync: sync-web
sync-web:
    yarn sync

# Clean commands
clean:
    cargo clean
    rm -rf node_modules
    rm -rf uploads
    rm -rf videos

# Verification commands
verify:
    cargo check
    cargo test
    cargo clippy -- -D warnings
    cargo fmt --all -- --check

# Nuke all data
nuke:
    @just drop-db
    @just clean

# Install all dependencies
# TODO: Install ffmpeg dependency
install:
    yarn
    cargo check

init:
    @just install
    @just init-db
