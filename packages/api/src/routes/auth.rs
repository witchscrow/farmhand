use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use db::users::User;
use serde::{Deserialize, Serialize};

use crate::{jwt::encode_jwt, AppState};

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    password_confirmation: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: Option<String>,
    email: Option<String>,
    password: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

/// A function for setting a JWT to a response cookie
fn get_auth_response(jwt_token: &str) -> impl IntoResponse {
    let mut response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap();
    let cookie = format!(
        "jwt={}; HttpOnly; Path=/; Max-Age=86400; SameSite=Strict{}",
        jwt_token,
        if cfg!(debug_assertions) {
            ""
        } else {
            "; Secure"
        }
    );
    let cookie_header = HeaderValue::from_str(&cookie).expect("Could not parse cookie header");

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie_header);

    response
}

/// Handle user registration with password hashing and validation
/// Returns a cookie with a JWT set on successful response
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    // Validate inputs
    if payload.password != payload.password_confirmation {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create a new user
    let mut user = User::new(payload.email, payload.username, payload.password);
    // Make sure to hash the password
    user.hash_password()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Insert the new user into the database
    match user.insert(&state.db).await {
        Ok(user) => {
            let token =
                encode_jwt(&user.id.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let response = get_auth_response(&token);
            Ok(response)
        }
        Err(_e) => Err(StatusCode::BAD_REQUEST),
    }
}

/// Handle user authentication
/// Returns a cookie with a JWT set on successful response
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // Make sure we have either a username or email to work with
    if payload.email.is_none() && payload.username.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    // Make sure the password isn't empty
    if payload.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    // Grab the user based on either the username or email
    let mut user: Option<User> = None;
    // If the username is provided, it supercedes the email
    if let Some(username) = payload.username {
        user = Some(
            User::from_username(username, &state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        );
    }
    // Otherwise, we use the email
    if let Some(email) = payload.email {
        user = Some(
            User::from_email(email, &state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        )
    }
    // Finally, check the passwords
    if let Some(user) = user {
        user.check_password(payload.password)
            .map_err(|_| StatusCode::BAD_REQUEST)?;and return it
        let token =
            encode_jwt(&user.id.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(get_auth_response(&token))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

/// Logout a user, clears the JWT cookie
pub async fn logout() -> Response {
    let mut response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap();

    let cookie = "jwt=; HttpOnly; Path=/; Max-Age=0; SameSite=Strict";
    let cookie_header = HeaderValue::from_str(cookie).unwrap();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie_header);

    response
}
