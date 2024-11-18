use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use db::users::{User, UserRole};
use serde::Serialize;

#[derive(Serialize)]
/// User data with sensitive data stripped out
struct UserResponse {
    username: String,
    email: String,
    role: UserRole,
}

/// Gets a user by their ID
pub async fn get_user(Extension(user): Extension<Option<User>>) -> impl IntoResponse {
    match user {
        Some(user) => Ok(Json(UserResponse {
            username: user.username,
            email: user.email,
            role: user.role,
        })),
        None => Err(StatusCode::BAD_REQUEST),
    }
}
