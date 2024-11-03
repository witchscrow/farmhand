use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub raw_video_path: String,
    pub processed_video_path: Option<String>,
    pub processing_status: ProcessingStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "processing_status", rename_all = "lowercase")]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl Video {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        title: String,
        raw_video_path: String,
    ) -> Result<Self, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO videos (user_id, title, raw_video_path, processing_status)
            VALUES ($1, $2, $3, 'pending')
            RETURNING id, user_id, title, raw_video_path, processed_video_path,
                      processing_status, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(title)
        .bind(raw_video_path)
        .map(|row: PgRow| Video {
            id: row.get("id"),
            user_id: row.get("user_id"),
            title: row.get("title"),
            raw_video_path: row.get("raw_video_path"),
            processed_video_path: row.get("processed_video_path"),
            processing_status: row.get("processing_status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}
