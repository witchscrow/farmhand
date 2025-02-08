use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
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
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Creator,
    Viewer,
}

impl ToString for UserRole {
    fn to_string(&self) -> String {
        match self {
            UserRole::Admin => "admin".to_string(),
            UserRole::Creator => "creator".to_string(),
            UserRole::Viewer => "viewer".to_string(),
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => UserRole::Admin,
            "creator" => UserRole::Creator,
            "viewer" => UserRole::Viewer,
            _ => {
                tracing::warn!("Invalid role: {}", s);
                UserRole::Viewer // Default to viewer role
            }
        }
    }
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
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
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
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool)
            .await
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
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hashed_password = argon2
            .hash_password(self.password_hash.as_bytes(), &salt)
            .map_err(|_| UserError::FailedToHashPassword)?
            .to_string();
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
