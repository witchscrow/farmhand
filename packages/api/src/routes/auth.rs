use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use db::users::{hash_password, insert_user};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    password_confirmation: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

// Handle user registration with password hashing and validation
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // Validate inputs
    if payload.password != payload.password_confirmation {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: "Passwords do not match".to_string(),
            }),
        ));
    }

    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: "All fields are required".to_string(),
            }),
        ));
    }

    // Hash the password
    let password_hash = hash_password(&payload.password).expect("Could not hash password");

    // Insert the new user into the database
    match insert_user(&state.db, &payload.email, &payload.username, &password_hash).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            let error_message = match e {
                sqlx::Error::Database(ref db_error) if db_error.is_unique_violation() => {
                    "Username or email already exists".to_string()
                }
                _ => "Failed to create user".to_string(),
            };

            Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    message: error_message,
                }),
            ))
        }
    }
}
