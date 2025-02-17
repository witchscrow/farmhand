pub mod accounts;
pub mod users;
pub mod videos;

pub use users::User;
pub use videos::{ProcessingStatus, Video};

use sqlx::{postgres::PgPool, Pool, Postgres};

pub type DBPool = Pool<Postgres>;

/// Function to establish a connection to the PostgreSQL database
pub async fn connect_to_database() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::debug!("Creating DB connection Pool");
    let pool = PgPool::connect(&database_url).await?;

    Ok(pool)
}

/// Function for running the database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::debug!("Running database migrations");
    sqlx::migrate!().run(pool).await?;

    Ok(())
}
/// Function for deleting all data from the database
pub async fn delete_all_data(pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::debug!("Deleting all data from the database");
    sqlx::query(
        "DO $$ DECLARE
            r RECORD;
        BEGIN
            -- Drop all tables
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
                EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;

            -- Drop all types/enums
            FOR r IN (SELECT typname FROM pg_type WHERE typnamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'public')) LOOP
                EXECUTE 'DROP TYPE IF EXISTS ' || quote_ident(r.typname) || ' CASCADE';
            END LOOP;

            -- Drop all functions/triggers
            FOR r IN (SELECT proname FROM pg_proc WHERE pronamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'public')) LOOP
                EXECUTE 'DROP FUNCTION IF EXISTS ' || quote_ident(r.proname) || ' CASCADE';
            END LOOP;

            -- Drop all sequences
            FOR r IN (SELECT sequence_name FROM information_schema.sequences WHERE sequence_schema = 'public') LOOP
                EXECUTE 'DROP SEQUENCE IF EXISTS ' || quote_ident(r.sequence_name) || ' CASCADE';
            END LOOP;
        END $$;"
    )
    .execute(pool)
    .await?;

    Ok(())
}
