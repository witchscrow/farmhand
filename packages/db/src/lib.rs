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
