use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Video {
    pub id: String,
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
    /// A function for creating new video data in the db
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        title: String,
        raw_video_path: String,
    ) -> Result<Self, sqlx::Error> {
        let video_id = nanoid!(10);
        sqlx::query_as::<_, Video>(
            r#"
            INSERT INTO videos (id, user_id, title, raw_video_path, processing_status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, user_id, title, raw_video_path, processed_video_path,
                      processing_status, created_at, updated_at
            "#,
        )
        .bind(video_id)
        .bind(user_id)
        .bind(title)
        .bind(raw_video_path)
        .fetch_one(pool)
        .await
    }
    /// A function for fetching video data from the db by video ID
    pub async fn by_id(pool: &PgPool, video_id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Video>(
            r#"
            SELECT id, user_id, title, raw_video_path, processed_video_path,
                   processing_status, created_at, updated_at
            FROM videos
            WHERE id = $1
            "#,
        )
        .bind(video_id)
        .fetch_optional(pool)
        .await
    }
    /// A function for getting all user owned videos by user ID
    pub async fn by_userid(pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Video>(
            r#"
            SELECT id, user_id, title, raw_video_path, processed_video_path,
                   processing_status, created_at, updated_at
            FROM videos
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }
    /// A function for getting all user owned videos by user name
    pub async fn by_username(pool: &PgPool, username: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Video>(
            r#"
            SELECT v.id, v.user_id, v.title, v.raw_video_path, v.processed_video_path,
                   v.processing_status, v.created_at, v.updated_at
            FROM videos v
            JOIN users u ON u.id = v.user_id
            WHERE u.name = $1
            ORDER BY v.created_at DESC
            "#,
        )
        .bind(username)
        .fetch_all(pool)
        .await
    }
}
