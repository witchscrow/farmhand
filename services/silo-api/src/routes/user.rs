use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use db::users::{User, UserRole};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize)]
/// User data with sensitive data stripped out
struct UserResponse {
    username: String,
    email: String,
    role: UserRole,
}

/// Gets the owner of the token used to authenticate
pub async fn get_self(Extension(user): Extension<Option<User>>) -> impl IntoResponse {
    match user {
        Some(user) => Ok(Json(UserResponse {
            username: user.username,
            email: user.email,
            role: user.role,
        })),
        None => Err(StatusCode::BAD_REQUEST),
    }
}

#[derive(Deserialize, Debug)]
pub struct UserByID {
    id: String,
}

#[derive(Deserialize, Debug)]
pub struct UserByUserName {
    name: String,
}

#[derive(Serialize)]
pub struct UserListResponse {
    users: Vec<UserResponse>,
}

/// Gets a user by specified query params
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    video_query: Option<Query<UserByID>>,
    username_query: Option<Query<UserByUserName>>,
) -> impl IntoResponse {
    tracing::debug!(
        "Got user get query params:\n\tuser_query: {:?}\n\tusername_query: {:?}",
        video_query,
        username_query
    );

    match (video_query, username_query) {
        // User by ID
        (Some(user_query), None) => {
            let id_to_find = uuid::Uuid::parse_str(&user_query.id).map_err(|e| {
                tracing::error!("Failed to parse uuid {e}");
                StatusCode::BAD_REQUEST
            })?;

            let found_user = User::by_id(id_to_find, &state.db).await.map_err(|e| {
                if let sqlx::Error::RowNotFound = e {
                    StatusCode::NOT_FOUND
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            })?;

            let res_user = UserResponse {
                username: found_user.username,
                email: found_user.email,
                role: found_user.role,
            };

            Ok(Json(UserListResponse {
                users: vec![res_user],
            }))
        }
        // User by username
        (None, Some(username_query)) => {
            let found_user = User::by_username(username_query.name.clone(), &state.db)
                .await
                .map_err(|e| {
                    tracing::error!("Error getting user by username {e}");
                    StatusCode::BAD_REQUEST
                })?;

            let res_user = UserResponse {
                username: found_user.username,
                email: found_user.email,
                role: found_user.role,
            };

            Ok(Json(UserListResponse {
                users: vec![res_user],
            }))
        }
        // User by ID and username (not implemented)
        (Some(_), Some(_)) => Err(StatusCode::NOT_IMPLEMENTED),
        // No query params, get all users
        (None, None) => {
            tracing::debug!("No queries provided, getting all users");
            let users = match User::all(&state.db).await {
                Ok(users) => users,
                Err(e) => {
                    tracing::error!("Error listing users: {e}");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };

            let users = users
                .into_iter()
                .map(|user| UserResponse {
                    username: user.username,
                    email: user.email,
                    role: user.role,
                })
                .collect();

            Ok(Json(UserListResponse { users }))
        }
    }
}
