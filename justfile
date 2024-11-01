set dotenv-load := true

start-api:
    cargo run -p api

start-web:
    yarn start

dev-api: start-api

dev-web:
    yarn dev

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

build-web:
    yarn build

build-api:
    cargo build -p api
