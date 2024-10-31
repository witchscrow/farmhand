set dotenv-load := true

start-api:
    cargo run -p api

start-web:
    yarn start

dev-web:
    yarn dev

init-db: create-db migrate

create-db:
    sqlx database create

drop-db:
    sqlx database drop

migrate:
    sqlx migrate run --source packages/db/migrations

revert:
    sqlx migrate run --source packages/db/migrations

build-web:
    yarn build
