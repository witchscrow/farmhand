use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::{types::Uuid, PgPool};

// Insert a new user into the database
pub async fn insert_user(
    pool: &PgPool,
    email: &str,
    username: &str,
    password: &str,
) -> Result<String, sqlx::Error> {
    let user_id = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, username, password_hash) VALUES ($1, $2, $3, $4)")
        .bind(user_id)
        .bind(email)
        .bind(username)
        .bind(password)
        .execute(pool)
        .await?;

    Ok(user_id.to_string())
}

// Hash a password using Argon2
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}
