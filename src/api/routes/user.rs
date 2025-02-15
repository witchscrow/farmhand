use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{
    api::{app_state::AppState, twitch::eventsub::subscribers::subscribe_to_events},
    db::{
        users::{UserRole, UserSettings},
        User,
    },
};

#[derive(Serialize)]
/// User data with sensitive data stripped out
pub struct UserResponse {
    username: String,
    email: String,
    role: UserRole,
    settings: Option<UserSettings>,
}

/// Gets the owner of the token used to authenticate
pub async fn get_self(Extension(user): Extension<Option<User>>) -> impl IntoResponse {
    match user {
        Some(user) => Ok(Json(UserResponse {
            username: user.username,
            email: user.email,
            role: user.role,
            settings: user.settings,
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
    Extension(_user): Extension<Option<User>>,
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
                settings: found_user.settings,
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
                settings: found_user.settings,
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
                    settings: user.settings,
                })
                .collect();

            Ok(Json(UserListResponse { users }))
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateUserSettings {
    stream_status_enabled: bool,
    chat_messages_enabled: bool,
    channel_points_enabled: bool,
    follows_subs_enabled: bool,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    username: String,
    settings: UpdateUserSettings,
}

#[derive(Debug)]
pub enum WebhookError {
    UserNotFound(String),
    CredentialsError(String),
    TwitchAccountMissing,
    SettingsMissing,
    TokenMissing,
    EventSubError(String),
}

impl std::fmt::Display for WebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookError::UserNotFound(e) => write!(f, "User not found: {}", e),
            WebhookError::CredentialsError(e) => write!(f, "Credentials error: {}", e),
            WebhookError::TwitchAccountMissing => write!(f, "User does not have a Twitch account"),
            WebhookError::SettingsMissing => write!(f, "User does not have settings"),
            WebhookError::TokenMissing => write!(f, "User does not have a Twitch access token"),
            WebhookError::EventSubError(e) => write!(f, "EventSub error: {}", e),
        }
    }
}

impl std::error::Error for WebhookError {}

/// Saves a user
#[axum::debug_handler]
pub async fn save_user(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    Json(post): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let Some(user) = user else {
        tracing::error!("Failed to get user");
        return Err(StatusCode::BAD_REQUEST);
    };

    if user.username != post.username {
        tracing::error!("User attempted to update another user's data");
        return Err(StatusCode::FORBIDDEN);
    }

    // Get current chat messages setting
    let was_chat_enabled = user
        .settings
        .as_ref()
        .and_then(|s| s.chat_messages_enabled)
        .is_some();

    // Update settings
    match user
        .clone()
        .update_settings(
            post.settings.stream_status_enabled,
            post.settings.chat_messages_enabled,
            post.settings.channel_points_enabled,
            post.settings.follows_subs_enabled,
            &state.db,
        )
        .await
    {
        Ok(settings) => {
            // Check if chat messages was newly enabled
            if post.settings.chat_messages_enabled && !was_chat_enabled {
                if let Err(e) = setup_chat_messages_webhook(user.id, &state.db).await {
                    tracing::error!("Failed to set up EventSub subscriptions: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }

            Ok(Json(UserResponse {
                username: user.username,
                email: user.email,
                role: user.role,
                settings: Some(settings.clone()),
            }))
        }
        Err(e) => {
            tracing::error!("Error updating settings: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Function to handle chat message webhook setup
async fn setup_chat_messages_webhook(
    user_id: uuid::Uuid,
    db: &Pool<Postgres>,
) -> Result<(), WebhookError> {
    let user = User::by_id(user_id, db)
        .await
        .map_err(|e| WebhookError::UserNotFound(e.to_string()))?;

    // Your webhook URL
    let webhook_url = format!("https://staging.api.farmhand.witchscrow.com/eventsub");

    let twitch_account = user
        .accounts
        .iter()
        .find(|account| account.provider == "twitch")
        .ok_or(WebhookError::TwitchAccountMissing)?;

    let settings = user.settings.ok_or(WebhookError::SettingsMissing)?;

    subscribe_to_events(
        user_id,
        twitch_account.provider_account_id.clone(),
        &settings,
        &webhook_url,
    )
    .await
    .map_err(|e| WebhookError::EventSubError(e.to_string()))?;

    tracing::info!(
        "Successfully set up EventSub subscriptions for user {}",
        user_id
    );

    Ok(())
}
