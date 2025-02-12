use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, PgPool};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
/// A database representation of a user
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    password_hash: String,
    pub role: UserRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    #[sqlx(skip)]
    pub settings: Option<UserSettings>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stream_status_enabled: Option<DateTime<Utc>>,
    pub chat_messages_enabled: Option<DateTime<Utc>>,
    pub channel_points_enabled: Option<DateTime<Utc>>,
    pub follows_subs_enabled: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct UserWithSettings {
    // User fields
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    // Settings fields
    pub settings_id: Uuid,
    pub stream_status_enabled: Option<DateTime<Utc>>,
    pub chat_messages_enabled: Option<DateTime<Utc>>,
    pub channel_points_enabled: Option<DateTime<Utc>>,
    pub follows_subs_enabled: Option<DateTime<Utc>>,
    pub settings_created_at: DateTime<Utc>,
    pub settings_updated_at: DateTime<Utc>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Creator,
    Viewer,
}

pub enum UserError {
    FailedToHashPassword,
    BadPassword,
}

impl User {
    /// Creates a new user from the given parameters
    // NOTE: This does not hash the password by default
    pub fn new(email: String, username: String, password_hash: String) -> Self {
        let id = Uuid::new_v4();
        User {
            id,
            email,
            username,
            password_hash,
            role: UserRole::Viewer,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            settings: None,
        }
    }
    /// Gets a user from the databased based on Username
    pub async fn by_username(username: String, pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await
    }
    /// Gets a user from the database based on ID
    pub async fn by_id(id: Uuid, pool: &PgPool) -> Result<Self, sqlx::Error> {
        let row = sqlx::query_as::<_, UserWithSettings>(
            r#"
                SELECT
                    u.*,
                    s.id as settings_id,
                    s.stream_status_enabled,
                    s.chat_messages_enabled,
                    s.channel_points_enabled,
                    s.follows_subs_enabled,
                    s.created_at as settings_created_at,
                    s.updated_at as settings_updated_at
                FROM users u
                LEFT JOIN user_settings s ON s.user_id = u.id
                WHERE u.id = $1
                "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(User {
            id: row.id,
            email: row.email,
            username: row.username,
            password_hash: row.password_hash,
            role: row.role,
            created_at: row.created_at,
            updated_at: row.updated_at,
            settings: Some(UserSettings {
                id: row.settings_id,
                user_id: row.id,
                stream_status_enabled: row.stream_status_enabled,
                chat_messages_enabled: row.chat_messages_enabled,
                channel_points_enabled: row.channel_points_enabled,
                follows_subs_enabled: row.follows_subs_enabled,
                created_at: row.settings_created_at,
                updated_at: row.settings_updated_at,
            }),
        })
    }
    /// Gets a user from the databased based on Email
    pub async fn by_email(email: String, pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(pool)
            .await
    }
    /// Gets all users from the database
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let rows = sqlx::query_as::<_, UserWithSettings>(
            r#"
                SELECT
                    u.*,
                    s.id as settings_id,
                    s.stream_status_enabled,
                    s.chat_messages_enabled,
                    s.channel_points_enabled,
                    s.follows_subs_enabled,
                    s.created_at as settings_created_at,
                    s.updated_at as settings_updated_at
                FROM users u
                LEFT JOIN user_settings s ON s.user_id = u.id
                "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| User {
                id: row.id,
                email: row.email,
                username: row.username,
                password_hash: row.password_hash,
                role: row.role,
                created_at: row.created_at,
                updated_at: row.updated_at,
                settings: Some(UserSettings {
                    id: row.settings_id,
                    user_id: row.id,
                    stream_status_enabled: row.stream_status_enabled,
                    chat_messages_enabled: row.chat_messages_enabled,
                    channel_points_enabled: row.channel_points_enabled,
                    follows_subs_enabled: row.follows_subs_enabled,
                    created_at: row.settings_created_at,
                    updated_at: row.settings_updated_at,
                }),
            })
            .collect())
    }
    /// Updates the user's settings
    pub async fn update_settings(
        &mut self,
        stream_status: bool,
        chat_messages: bool,
        channel_points: bool,
        follows_subs: bool,
        pool: &PgPool,
    ) -> Result<&UserSettings, sqlx::Error> {
        let now = Utc::now();

        let settings = sqlx::query_as::<_, UserSettings>(
            r#"
                UPDATE user_settings
                SET
                    stream_status_enabled = CASE WHEN $1 THEN $5 ELSE NULL END,
                    chat_messages_enabled = CASE WHEN $2 THEN $5 ELSE NULL END,
                    channel_points_enabled = CASE WHEN $3 THEN $5 ELSE NULL END,
                    follows_subs_enabled = CASE WHEN $4 THEN $5 ELSE NULL END,
                    updated_at = $5
                WHERE user_id = $6
                RETURNING *
                "#,
        )
        .bind(stream_status)
        .bind(chat_messages)
        .bind(channel_points)
        .bind(follows_subs)
        .bind(now)
        .bind(self.id)
        .fetch_one(pool)
        .await?;

        self.settings = Some(settings);
        Ok(self.settings.as_ref().unwrap())
    }
    /// Checks that the raw password matches the expected-to-be-hashed password
    pub fn check_password(&self, raw_password: String) -> Result<(), UserError> {
        let dashed_password = argon2::password_hash::PasswordHash::new(&self.password_hash)
            .map_err(|_| UserError::FailedToHashPassword)?;
        argon2::Argon2::default()
            .verify_password(raw_password.as_bytes(), &dashed_password)
            .map_err(|_| UserError::BadPassword)
    }
    /// Hashes the password for the user
    pub fn hash_password(&mut self) -> Result<&mut Self, UserError> {
        let hashed_password =
            hash_string(&self.password_hash).map_err(|_| UserError::FailedToHashPassword)?;
        self.password_hash = hashed_password;
        Ok(self)
    }
    /// Inserts the user into the database
    pub async fn insert(&self, pool: &PgPool) -> Result<&Self, sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (id, email, username, password_hash, role) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&self.id)
        .bind(&self.email)
        .bind(&self.username)
        .bind(&self.password_hash)
        .bind(&self.role)
        .execute(pool)
        .await?;

        Ok(self)
    }
}

/// Hash a string using Argon2
pub fn hash_string(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}
