use crate::{Job, Message};
use sqlx::types::Json;
use uuid::Uuid;

/// A Postgres representation of a Job
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct PostgresJob {
    id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,

    scheduled_for: chrono::DateTime<chrono::Utc>,
    failed_attempts: i32,
    status: PostgresJobStatus,
    message: Json<Message>,
}

/// The different status' that a job can be in
// We use a INT as Postgres representation for performance reasons
#[derive(Debug, Clone, sqlx::Type, PartialEq)]
#[repr(i32)]
pub enum PostgresJobStatus {
    Queued,
    Running,
    Failed,
}

impl From<PostgresJob> for Job {
    fn from(item: PostgresJob) -> Self {
        Job {
            id: item.id,
            message: item.message.0,
        }
    }
}
