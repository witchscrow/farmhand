use axum::http::header;
use common::db::users::UserSettings;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::{auth::oauth::twitch::TwitchCredentials, user::WebhookError};

const TWITCH_API_URL: &str = "https://api.twitch.tv/helix/eventsub/subscriptions";

#[derive(Debug, Serialize)]
struct EventSubCondition {
    broadcaster_user_id: String,
    // Add other condition fields as needed
}

#[derive(Debug, Serialize)]
struct EventSubTransport {
    method: String,
    callback: String,
    secret: String,
}

#[derive(Debug, Serialize)]
struct EventSubRequest {
    #[serde(rename = "type")]
    event_type: String,
    version: String,
    condition: EventSubCondition,
    transport: EventSubTransport,
}

#[derive(Debug, Deserialize)]
struct EventSubResponse {
    data: Vec<EventSubData>,
}

#[derive(Debug, Deserialize)]
struct EventSubData {
    id: String,
    status: String,
}

pub async fn subscribe_to_events(
    user_id: Uuid,
    twitch_user_id: String,
    settings: &UserSettings,
    webhook_url: &str,
) -> Result<(), WebhookError> {
    // Get Twitch credentials and app access token
    let credentials = TwitchCredentials::from_env().map_err(|e| WebhookError::EventSubError(e))?;

    let app_access_token = credentials
        .get_app_access_token()
        .await
        .map_err(|e| WebhookError::EventSubError(e))?;

    let client = Client::new();
    let secret = TwitchCredentials::get_twitch_secret()
        .ok_or("Failed to get Twitch secret")
        .map_err(|e| WebhookError::EventSubError(e.to_string()))?;

    // Create a vector to store all subscription tasks
    let mut subscription_tasks = Vec::new();

    // Subscribe to different event types based on settings
    if settings.stream_status_enabled.is_some() {
        subscription_tasks.push(subscribe_to_event(
            &client,
            "stream.online",
            "1",
            &twitch_user_id,
            webhook_url,
            &secret,
            &credentials.id,
            &app_access_token,
        ));

        subscription_tasks.push(subscribe_to_event(
            &client,
            "stream.offline",
            "1",
            &twitch_user_id,
            webhook_url,
            &secret,
            &credentials.id,
            &app_access_token,
        ));
    }

    if settings.chat_messages_enabled.is_some() {
        subscription_tasks.push(subscribe_to_event(
            &client,
            "channel.chat.message",
            "1",
            &twitch_user_id,
            webhook_url,
            &secret,
            &credentials.id,
            &app_access_token,
        ));
    }

    if settings.channel_points_enabled.is_some() {
        subscription_tasks.push(subscribe_to_event(
            &client,
            "channel.channel_points_custom_reward_redemption.add",
            "1",
            &twitch_user_id,
            webhook_url,
            &secret,
            &credentials.id,
            &app_access_token,
        ));
    }

    if settings.follows_subs_enabled.is_some() {
        subscription_tasks.push(subscribe_to_event(
            &client,
            "channel.follow",
            "2",
            &twitch_user_id,
            webhook_url,
            &secret,
            &credentials.id,
            &app_access_token,
        ));

        subscription_tasks.push(subscribe_to_event(
            &client,
            "channel.subscribe",
            "1",
            &twitch_user_id,
            webhook_url,
            &secret,
            &credentials.id,
            &app_access_token,
        ));
    }

    // Execute all subscription requests concurrently
    let results = futures::future::join_all(subscription_tasks).await;

    // Check for any errors
    for result in results {
        if let Err(e) = result {
            tracing::error!("Failed to subscribe to event: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

async fn subscribe_to_event(
    client: &Client,
    event_type: &str,
    version: &str,
    broadcaster_id: &str,
    webhook_url: &str,
    secret: &str,
    client_id: &str,
    access_token: &str,
) -> Result<(), WebhookError> {
    let request = EventSubRequest {
        event_type: event_type.to_string(),
        version: version.to_string(),
        condition: EventSubCondition {
            broadcaster_user_id: broadcaster_id.to_string(),
        },
        transport: EventSubTransport {
            method: "webhook".to_string(),
            callback: webhook_url.to_string(),
            secret: secret.to_string(),
        },
    };

    let response = client
        .post(TWITCH_API_URL)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Client-Id", client_id)
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&request)
        .send()
        .await
        .map_err(|e| WebhookError::EventSubError(e.to_string()))?;

    if !response.status().is_success() {
        let response_error = response
            .text()
            .await
            .map_err(|e| WebhookError::EventSubError(e.to_string()))?;
        tracing::error!(
            "Failed to subscribe to event {}: {}",
            event_type,
            response_error
        );
        return Err(WebhookError::EventSubError(response_error));
    }

    let subscription: EventSubResponse = response
        .json()
        .await
        .map_err(|e| WebhookError::EventSubError(e.to_string()))?;
    tracing::info!(
        "Successfully subscribed to {} event: {:?}",
        event_type,
        subscription
    );

    Ok(())
}
