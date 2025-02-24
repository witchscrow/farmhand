use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;

use crate::{
    api::{app_state::AppState, routes::auth::oauth::twitch::TwitchCredentials},
    vendors::ChatMessagePayload,
    workers::events::Event,
};

type HmacSha256 = Hmac<Sha256>;
const HMAC_PREFIX: &str = "sha256=";

#[derive(Debug, Deserialize, Serialize)]
struct Subscription {
    id: String,
    status: String,
    #[serde(rename = "type")]
    event_type: String,
    version: String,
    cost: i32,
    condition: serde_json::Value,
    transport: Transport,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Transport {
    method: String,
    callback: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Notification {
    subscription: Subscription,
    event: Option<serde_json::Value>,
    challenge: Option<String>,
}

pub async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    // Extract headers
    let message_id = headers
        .get("twitch-eventsub-message-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let timestamp = headers
        .get("twitch-eventsub-message-timestamp")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let signature = headers
        .get("twitch-eventsub-message-signature")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let message_type = headers
        .get("twitch-eventsub-message-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    // Verify signature
    let hmac_message = format!(
        "{}{}{}",
        message_id,
        timestamp,
        String::from_utf8_lossy(&body)
    );

    let Some(twitch_secret) = TwitchCredentials::get_twitch_secret() else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Twitch secret not found").into_response();
    };
    if !verify_signature(&twitch_secret, &hmac_message, signature) {
        return (StatusCode::FORBIDDEN, "Invalid signature").into_response();
    }

    // Parse notification
    let notification: Notification = match serde_json::from_slice(&body) {
        Ok(n) => n,
        Err(e) => {
            tracing::error!("Failed to parse notification: {}", e);
            return (StatusCode::BAD_REQUEST, "Invalid request body").into_response();
        }
    };

    // Handle different message types
    match message_type {
        "notification" => {
            tracing::debug!("Event type: {}", notification.subscription.event_type);
            let notification_type = notification.subscription.event_type;
            match notification_type.as_str() {
                "stream.online" => {
                    return (StatusCode::NOT_IMPLEMENTED, "Stream online not implemented")
                        .into_response()
                }
                "stream.offline" => {
                    return (
                        StatusCode::NOT_IMPLEMENTED,
                        "Stream offline not implemented",
                    )
                        .into_response()
                }
                "channel.follow" => {
                    return (
                        StatusCode::NOT_IMPLEMENTED,
                        "Channel follow not implemented",
                    )
                        .into_response()
                }
                "channel.subscribe" => {
                    return (
                        StatusCode::NOT_IMPLEMENTED,
                        "Channel subscription not implemented",
                    )
                        .into_response()
                }
                "channel.chat.message" => {
                    tracing::debug!("Channel chat message received");
                    // Pull the raw payload out of the notification
                    let Some(raw_payload) = notification.event else {
                        tracing::error!("Received channel.chat.message notification without event");
                        return (StatusCode::BAD_REQUEST, "Missing event data").into_response();
                    };
                    // Parse into a ChatMessagePayload so we can get the appropriate subject
                    let message_payload =
                        serde_json::from_value::<ChatMessagePayload>(raw_payload.clone());
                    let Ok(message_payload) = message_payload else {
                        tracing::error!("Failed to parse channel.chat.message notification");
                        return (StatusCode::BAD_REQUEST, "Invalid event data").into_response();
                    };
                    // Get the subject
                    let subject = Event::from(message_payload).get_subject();
                    state
                        .event_stream
                        .publish(
                            subject.to_string(),
                            raw_payload.to_string(), // Pass the original payload so we can skip serialization
                        )
                        .await
                        .map_err(|e| {
                            tracing::error!("Failed to publish chat message job: {}", e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })
                        .expect("Failed to publish chat message job");
                }
                _ => {
                    tracing::warn!("Unhandled notification event type: {}", notification_type);
                }
            }
            StatusCode::NO_CONTENT.into_response()
        }
        "webhook_callback_verification" => {
            if let Some(challenge) = notification.challenge {
                (
                    StatusCode::OK,
                    [(axum::http::header::CONTENT_TYPE, "text/plain")],
                    challenge,
                )
                    .into_response()
            } else {
                StatusCode::BAD_REQUEST.into_response()
            }
        }
        "revocation" => {
            tracing::debug!(
                "{} notifications revoked!",
                notification.subscription.event_type
            );
            tracing::debug!("reason: {}", notification.subscription.status);
            tracing::debug!(
                "condition: {}",
                serde_json::to_string_pretty(&notification.subscription.condition).unwrap()
            );
            StatusCode::NO_CONTENT.into_response()
        }
        _ => {
            tracing::debug!("Unknown message type: {}", message_type);
            StatusCode::NO_CONTENT.into_response()
        }
    }
}

fn verify_signature(secret: &str, message: &str, signature: &str) -> bool {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");

    mac.update(message.as_bytes());
    let result = mac.finalize().into_bytes();
    let computed_signature = format!("{}{:x}", HMAC_PREFIX, result);
    tracing::debug!("Computed signature: {}", computed_signature);
    tracing::debug!("Expected signature: {}", signature);

    // Constant-time comparison
    computed_signature == signature
}
