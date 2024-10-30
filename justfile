set dotenv-load := true

start-api:
    cargo run -p api

start-web:
    yarn start

dev-web:
    yarn dev
