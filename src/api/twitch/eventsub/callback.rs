use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use bytes::Bytes;
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::Arc;

use crate::{
    api::{app_state::AppState, routes::auth::oauth::twitch::TwitchCredentials},
    db::streams::Stream,
    event::Event,
    twitch::{subscription::Notification, ChatMessagePayload, StreamStatusPayload},
};

type HmacSha256 = Hmac<Sha256>;
const HMAC_PREFIX: &str = "sha256=";

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
                    let Some(raw_payload) = notification.event else {
                        tracing::error!("Received stream status notification without event");
                        return (StatusCode::BAD_REQUEST, "Missing event data").into_response();
                    };

                    let stream_payload =
                        match serde_json::from_value::<StreamStatusPayload>(raw_payload) {
                            Ok(payload) => payload,
                            Err(err) => {
                                tracing::error!(
                                    "Failed to parse stream status notification: {}",
                                    err
                                );
                                return (StatusCode::BAD_REQUEST, "Invalid event data")
                                    .into_response();
                            }
                        };

                    // Start by getting the user account by the payload
                    let Ok(user_account) = stream_payload.find_broadcaster_account(&state.db).await
                    else {
                        tracing::error!("Failed to find broadcaster account");
                        return (
                            StatusCode::BAD_REQUEST,
                            "Failed to find broadcaster account",
                        )
                            .into_response();
                    };
                    // Parse the start time from the payload
                    let Some(start_time) = stream_payload.started_at() else {
                        tracing::error!("Failed to find stream start time");
                        return (StatusCode::BAD_REQUEST, "Failed to find stream start time")
                            .into_response();
                    };
                    // Save the stream to the database
                    let stream = match Stream::create(user_account.user_id, start_time, &state.db)
                        .await
                    {
                        Ok(stream) => stream,
                        Err(err) => {
                            tracing::error!("Failed to create stream: {}", err);
                            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create stream")
                                .into_response();
                        }
                    };

                    // Lastly, publish the stream status event
                    let event = Event::from(stream_payload).set_stream_db_id(stream.id);
                    let subject = event.get_subject();
                    let Ok(payload) = serde_json::to_string(&event) else {
                        tracing::error!("Failed to serialize event payload");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to serialize event payload",
                        )
                            .into_response();
                    };
                    state
                        .event_stream
                        .publish(subject, payload)
                        .await
                        .map_err(|e| {
                            tracing::error!("Failed to publish stream status event: {}", e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })
                        .expect("Failed to publish stream status event");
                }
                "stream.offline" => {
                    let Some(raw_payload) = notification.event else {
                        tracing::error!("Received stream status notification without event");
                        return (StatusCode::BAD_REQUEST, "Missing event data").into_response();
                    };

                    let stream_payload =
                        match serde_json::from_value::<StreamStatusPayload>(raw_payload) {
                            Ok(payload) => payload,
                            Err(err) => {
                                tracing::error!(
                                    "Failed to parse stream status notification: {}",
                                    err
                                );
                                return (StatusCode::BAD_REQUEST, "Invalid event data")
                                    .into_response();
                            }
                        };

                    // Start by getting the user account by the payload
                    let Ok(user_account) = stream_payload.find_broadcaster_account(&state.db).await
                    else {
                        tracing::error!("Failed to find broadcaster account");
                        return (
                            StatusCode::BAD_REQUEST,
                            "Failed to find broadcaster account",
                        )
                            .into_response();
                    };
                    // Get the last active stream for the user
                    let Ok(last_active_stream) =
                        Stream::find_most_recent_active_by_user_id(user_account.user_id, &state.db)
                            .await
                    else {
                        tracing::error!("Failed to find last active stream");
                        return (StatusCode::BAD_REQUEST, "Failed to find last active stream")
                            .into_response();
                    };
                    // If there is not an active stream, something went wrong
                    let Some(mut stream) = last_active_stream else {
                        tracing::error!(
                            "Failed to find last active stream for user: {}",
                            user_account.user_id
                        );
                        return (StatusCode::BAD_REQUEST, "Failed to find last active stream")
                            .into_response();
                    };
                    // Update the end time of the stream
                    let end_time = Utc::now();
                    let Ok(stream) = stream.end_stream(end_time, &state.db).await else {
                        tracing::error!("Failed to end stream, could not update database");
                        return (StatusCode::BAD_REQUEST, "Failed to end stream").into_response();
                    };

                    // Lastly, publish the stream status event
                    let event = Event::from(stream_payload).set_stream_db_id(stream.id);
                    let subject = event.get_subject();
                    let Ok(payload) = serde_json::to_string(&event) else {
                        tracing::error!("Failed to serialize stream status event");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to serialize stream status event",
                        )
                            .into_response();
                    };
                    state
                        .event_stream
                        .publish(subject, payload)
                        .await
                        .map_err(|e| {
                            tracing::error!("Failed to publish stream status event: {}", e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })
                        .expect("Failed to publish stream status event");
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
