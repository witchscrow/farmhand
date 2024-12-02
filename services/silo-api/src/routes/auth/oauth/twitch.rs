use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
    Extension,
};
use chrono::{Duration, Utc};
use db::{accounts::Account, users::User};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use urlencoding::encode;
use uuid::Uuid;

use crate::{jwt::encode_jwt, AppState};

#[derive(Debug, Deserialize)]
pub struct TwitchCallback {
    code: String,
    scope: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TwitchCredentials {
    pub id: String,
    pub secret: String,
    pub redirect_uri: String,
    pub scope: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TwitchAccessTokens {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: Vec<String>,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct TwitchUserInfo {
    pub id: String,
    pub login: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: i32,
    pub email: String,
    pub created_at: String,
}

const BASE_OAUTH_URL: &str = "https://id.twitch.tv/oauth2/authorize";
const ENABLED_SCOPES: [&str; 3] = ["channel:bot", "user:read:email", "user:read:chat"];

impl TwitchCredentials {
    pub fn from_env() -> Result<Self, String> {
        Ok(TwitchCredentials {
            id: env::var("TWITCH_CLIENT_ID").map_err(|_| "Missing TWITCH_CLIENT_ID")?,
            secret: env::var("TWITCH_CLIENT_SECRET").map_err(|_| "Missing TWITCH_CLIENT_SECRET")?,
            redirect_uri: env::var("TWITCH_REDIRECT_URI")
                .map_err(|_| "Missing TWITCH_REDIRECT_URI")?,
            scope: ENABLED_SCOPES.join(" "),
        })
    }

    pub fn generate_oauth_url(&self) -> String {
        let params = [
            ("response_type", "code"),
            ("client_id", &self.id),
            ("redirect_uri", &self.redirect_uri),
            ("scope", &self.scope),
        ]
        .iter()
        .map(|(k, v)| format!("{}={}", k, encode(v)))
        .collect::<Vec<_>>()
        .join("&");

        format!("{}?{}", BASE_OAUTH_URL, params)
    }

    pub async fn get_access_tokens(&self, code: &str) -> Result<TwitchAccessTokens, String> {
        let client = Client::new();
        let response = client
            .post("https://id.twitch.tv/oauth2/token")
            .form(&[
                ("client_id", &self.id),
                ("client_secret", &self.secret),
                ("code", &code.to_string()),
                ("grant_type", &"authorization_code".to_string()),
                ("redirect_uri", &self.redirect_uri),
            ])
            .send()
            .await
            .map_err(|e| format!("Failed to send token request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Twitch returned error status: {}",
                response.status()
            ));
        }

        response
            .json::<TwitchAccessTokens>()
            .await
            .map_err(|e| format!("Failed to parse token response: {}", e))
    }

    pub async fn get_user_info(&self, access_token: &str) -> Result<TwitchUserInfo, String> {
        let client = Client::new();
        let response = client
            .get("https://api.twitch.tv/helix/users")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Client-Id", &self.id)
            .send()
            .await
            .map_err(|e| format!("Failed to get user info: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Twitch API error: {}", response.status()));
        }

        #[derive(Deserialize)]
        struct Response {
            data: Vec<TwitchUserInfo>,
        }

        let data: Response = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse user info: {}", e))?;

        data.data
            .into_iter()
            .next()
            .ok_or_else(|| "No user data in response".to_string())
    }
}

pub async fn oauth_redirect() -> Result<Redirect, StatusCode> {
    match TwitchCredentials::from_env() {
        Ok(creds) => Ok(Redirect::to(&creds.generate_oauth_url())),
        Err(_e) => {
            tracing::error!("Failed to initialize Twitch OAuth: {}", _e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<Option<User>>,
    Query(params): Query<TwitchCallback>,
) -> Result<Redirect, StatusCode> {
    // Initialize Twitch credentials
    let credentials = match TwitchCredentials::from_env() {
        Ok(creds) => creds,
        Err(e) => {
            tracing::error!("Failed to get Twitch credentials: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Get tokens and user info
    let tokens = match credentials.get_access_tokens(&params.code).await {
        Ok(tokens) => tokens,
        Err(e) => {
            tracing::error!("Failed to get access tokens: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let twitch_user = match credentials.get_user_info(&tokens.access_token).await {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Failed to get Twitch user info: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let expires_at = Utc::now() + Duration::seconds(tokens.expires_in as i64);

    let frontend_url = std::env::var("FRONTEND_URL").expect("Could not find frontend url");

    match current_user {
        // Update existing user's Twitch connection
        Some(user) => {
            if let Err(e) = Account::upsert(
                user.id,
                "twitch",
                &twitch_user.id,
                &tokens.access_token,
                &tokens.refresh_token,
                expires_at,
                &twitch_user.login,
                &state.db,
            )
            .await
            {
                tracing::error!("Failed to update Twitch account: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }

            // Generate JWT for existing user
            let token =
                encode_jwt(&user.id.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Redirect::to(&format!(
                "{}/login?token={}",
                frontend_url, token
            )))
        }
        // Create new user and Twitch connection
        None => {
            let user = if let Ok(existing_user) =
                User::by_email(twitch_user.email.clone(), &state.db).await
            {
                // Link Twitch account to existing user
                if let Err(e) = Account::upsert(
                    existing_user.id,
                    "twitch",
                    &twitch_user.id,
                    &tokens.access_token,
                    &tokens.refresh_token,
                    expires_at,
                    &twitch_user.login,
                    &state.db,
                )
                .await
                {
                    tracing::error!("Failed to link Twitch account to existing user: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
                existing_user
            } else {
                // Create new user
                let new_user = User::new(
                    twitch_user.email.clone(),
                    twitch_user.login.clone(),
                    Uuid::new_v4().to_string(), // Generate a random password
                );

                if let Err(e) = new_user.insert(&state.db).await {
                    tracing::error!("Failed to create new user: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }

                // Create Twitch account connection
                if let Err(e) = Account::create(
                    new_user.id,
                    "twitch",
                    &twitch_user.id,
                    &tokens.access_token,
                    &tokens.refresh_token,
                    expires_at,
                    &twitch_user.login,
                    &state.db,
                )
                .await
                {
                    tracing::error!("Failed to create Twitch account: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
                new_user
            };

            // Generate JWT for new or existing user
            let token =
                encode_jwt(&user.id.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Redirect::to(&format!(
                "{}/login?token={}",
                frontend_url, token
            )))
        }
    }
}
