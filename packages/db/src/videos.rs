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
    /// A function for generating a video id
    pub fn gen_id() -> String {
        nanoid!(10)
    }
    /// A function for creating new video data in the db
    pub async fn create(
        pool: &PgPool,
        video_id: Option<String>,
        user_id: Uuid,
        title: String,
        raw_video_path: Option<String>,
    ) -> Result<Self, sqlx::Error> {
        let video_id = video_id.unwrap_or(Self::gen_id());
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
    /// A function for fetching multiple videos from the db by video IDs
    pub async fn by_ids(pool: &PgPool, video_ids: &Vec<String>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Video>(
            r#"
            SELECT id, user_id, title, raw_video_path, processed_video_path,
                   processing_status, created_at, updated_at
            FROM videos
            WHERE id = ANY($1)
            "#,
        )
        .bind(video_ids)
        .fetch_all(pool)
        .await
    }
    /// A function for fetching a single video from the db by video ID
    pub async fn by_id(pool: &PgPool, video_id: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Video>(
            r#"
            SELECT id, user_id, title, raw_video_path, processed_video_path,
                   processing_status, created_at, updated_at
            FROM videos
            WHERE id = $1
            "#,
        )
        .bind(video_id)
        .fetch_one(pool)
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
    /// A function for getting all videos
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Video>(
            r#"
                SELECT id, user_id, title, raw_video_path, processed_video_path,
                       processing_status, created_at, updated_at
                FROM videos
                ORDER BY created_at DESC
                "#,
        )
        .fetch_all(pool)
        .await
    }
    /// A function for deleting videos by ID
    /// NOTE: Only a video owner can delete their video
    pub async fn delete(
        pool: &PgPool,
        user_id: Uuid,
        delete_list: Vec<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                    DELETE FROM videos
                    WHERE id = ANY($1)
                    AND user_id = $2
                    "#,
        )
        .bind(delete_list)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }
    /// A function for updating a videos processing status
    pub async fn update_status(
        pool: &PgPool,
        id: String,
        status: ProcessingStatus,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                UPDATE videos
                SET processing_status = $1
                WHERE id = $2
            "#,
        )
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }
}
