use crate::error::Error;
use crate::job::{PostgresJob, PostgresJobStatus};
use serde::{Deserialize, Serialize};
use sqlx::{self, types::Json, PgPool};
use std::fmt::Debug;
use ulid::Ulid;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait Queue: Send + Sync + Debug {
    /// pushes a job to the queue
    async fn push(
        &self,
        job: Message,
        status: PostgresJobStatus,
        scheduled_for: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), Error>;
    /// pull fetches at most `number_of_jobs` from the queue.
    async fn pull(&self, number_of_jobs: i32) -> Result<Vec<Job>, Error>;
    /// deletes a job from the queue
    async fn delete_job(&self, job_id: Uuid) -> Result<(), Error>;
    /// fails a job in the queue
    async fn fail_job(&self, job_id: Uuid) -> Result<(), Error>;
    /// clears the queue
    async fn clear(&self) -> Result<(), Error>;
}

/// The job to be processed, containing the message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub message: Message,
}

/// The payload of the job, containing the different jobs and their required data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    ProcessRawVideoIntoStream { video_id: String },
    CompressRawVideo { video_id: String },
}

/// The queue itself
#[derive(Debug, Clone)]
pub struct PostgresQueue {
    db: PgPool,
    max_attempts: u32,
}

impl PostgresQueue {
    pub fn new(db: PgPool) -> PostgresQueue {
        let queue = PostgresQueue {
            db,
            max_attempts: 5,
        };

        queue
    }
}

#[async_trait::async_trait]
impl Queue for PostgresQueue {
    async fn push(
        &self,
        job: Message,
        status: PostgresJobStatus,
        date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), Error> {
        let scheduled_for = date.unwrap_or(chrono::Utc::now());
        let failed_attempts: i32 = 0;
        let message = Json(job);
        let now = chrono::Utc::now();
        let job_id: Uuid = Ulid::new().into();
        let query = "INSERT INTO queue
            (id, created_at, updated_at, scheduled_for, failed_attempts, status, message)
            VALUES ($1, $2, $3, $4, $5, $6, $7)";
        tracing::debug!(
            "Adding job to queue with id: {}, status: {:?}, scheduled_for: {}",
            job_id,
            status,
            scheduled_for
        );
        sqlx::query(query)
            .bind(job_id)
            .bind(now)
            .bind(now)
            .bind(scheduled_for)
            .bind(failed_attempts)
            .bind(status)
            .bind(message)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn delete_job(&self, job_id: Uuid) -> Result<(), Error> {
        let query = "DELETE FROM queue WHERE id = $1";

        sqlx::query(query).bind(job_id).execute(&self.db).await?;
        Ok(())
    }

    async fn fail_job(&self, job_id: Uuid) -> Result<(), Error> {
        let now = chrono::Utc::now();

        // First get the current failed_attempts count
        let query = "SELECT failed_attempts FROM queue WHERE id = $1";
        let failed_attempts: i32 = sqlx::query_scalar(query)
            .bind(job_id)
            .fetch_one(&self.db)
            .await?;

        tracing::debug!(
            "Failing job with id: {}, attempt {} of {}",
            job_id,
            failed_attempts + 1,
            self.max_attempts
        );
        // Determine the new status based on failed attempts
        let new_status = if failed_attempts + 1 >= self.max_attempts as i32 {
            PostgresJobStatus::Failed
        } else {
            PostgresJobStatus::Queued
        };

        // Update the job with new status and increment failed_attempts
        let query = "UPDATE queue
            SET status = $1, updated_at = $2, failed_attempts = failed_attempts + 1
            WHERE id = $3";

        sqlx::query(query)
            .bind(new_status)
            .bind(now)
            .bind(job_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn pull(&self, number_of_jobs: i32) -> Result<Vec<Job>, Error> {
        let number_of_jobs = if number_of_jobs > 100 {
            100
        } else {
            number_of_jobs
        };
        let now = chrono::Utc::now();
        let query = "UPDATE queue
            SET status = $1, updated_at = $2
            WHERE id IN (
                SELECT id
                FROM queue
                WHERE status = ANY($3) AND scheduled_for <= $4 AND failed_attempts < $5
                ORDER BY scheduled_for
                FOR UPDATE SKIP LOCKED
                LIMIT $6
            )
            RETURNING *";

        let jobs: Vec<PostgresJob> = sqlx::query_as::<_, PostgresJob>(query)
            .bind(PostgresJobStatus::Running)
            .bind(now)
            .bind(vec![PostgresJobStatus::Queued])
            .bind(now)
            .bind(self.max_attempts as i32)
            .bind(number_of_jobs)
            .fetch_all(&self.db)
            .await?;

        Ok(jobs.into_iter().map(Into::into).collect())
    }

    async fn clear(&self) -> Result<(), Error> {
        let query = "DELETE FROM queue";

        sqlx::query(query).execute(&self.db).await?;
        Ok(())
    }
}
