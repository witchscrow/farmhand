use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use sqlx::{types::Uuid, PgPool};

#[derive(sqlx::FromRow)]
/// A database representation of a user
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
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
        }
    }
    /// Gets a user from the databased based on Username
    pub async fn from_username(username: String, pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await
    }
    /// Gets a user from the database based on ID
    pub async fn from_id(id: Uuid, pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }
    /// Gets a user from the databased based on Email
    pub async fn from_email(email: String, pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(pool)
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
        let hashed_password =
            hash_string(&self.password_hash).map_err(|_| UserError::FailedToHashPassword)?;
        self.password_hash = hashed_password;
        Ok(self)
    }
    /// Inserts the user into the database
    pub async fn insert(&self, pool: &PgPool) -> Result<&Self, sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (id, email, username, password_hash) VALUES ($1, $2, $3, $4)",
        )
        .bind(&self.id)
        .bind(&self.email)
        .bind(&self.username)
        .bind(&self.password_hash)
        .execute(pool)
        .await?;

        Ok(self)
    }
}

/// Hash a password using Argon2
pub fn hash_string(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}
