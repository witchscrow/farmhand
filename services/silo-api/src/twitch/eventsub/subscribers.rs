use axum::http::header;
use common::db::users::UserSettings;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::{auth::oauth::twitch::TwitchCredentials, user::WebhookError};

const TWITCH_API_URL: &str = "https://api.twitch.tv/helix/eventsub/subscriptions";

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum EventSubCondition {
    Basic {
        broadcaster_user_id: String,
    },
    ChatMessage {
        broadcaster_user_id: String,
        user_id: String,
    },
    Follow {
        broadcaster_user_id: String,
        moderator_user_id: String,
    },
    Subscribe {
        broadcaster_user_id: String,
    },
    ChannelPoints {
        broadcaster_user_id: String,
    },
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
struct EventSubSubscription {
    id: String,
    status: String,
    #[serde(rename = "type")]
    event_type: String,
    condition: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct EventSubListResponse {
    data: Vec<EventSubSubscription>,
    total: i32,
    total_cost: i32,
    max_total_cost: i32,
}

#[derive(Debug, Deserialize)]
struct EventSubData {
    id: String,
    status: String,
}

async fn list_subscriptions(
    client_id: &str,
    app_access_token: &str,
) -> Result<EventSubListResponse, WebhookError> {
    let client = Client::new();
    let response = client
        .get(TWITCH_API_URL)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Client-Id", client_id)
        .header("Authorization", format!("Bearer {}", app_access_token))
        .send()
        .await
        .map_err(|e| WebhookError::EventSubError(e.to_string()))?;

    if !response.status().is_success() {
        let error = response
            .text()
            .await
            .map_err(|e| WebhookError::EventSubError(e.to_string()))?;
        return Err(WebhookError::EventSubError(error));
    }

    response
        .json()
        .await
        .map_err(|e| WebhookError::EventSubError(e.to_string()))
}

async fn delete_subscription(
    sub_id: &str,
    client_id: &str,
    app_access_token: &str,
) -> Result<(), WebhookError> {
    let client = Client::new();
    let response = client
        .delete(&format!("{}/{}", TWITCH_API_URL, sub_id))
        .header(header::CONTENT_TYPE, "application/json")
        .header("Client-Id", client_id)
        .header("Authorization", format!("Bearer {}", app_access_token))
        .send()
        .await
        .map_err(|e| WebhookError::EventSubError(e.to_string()))?;

    if !response.status().is_success() {
        let error = response
            .text()
            .await
            .map_err(|e| WebhookError::EventSubError(e.to_string()))?;
        return Err(WebhookError::EventSubError(error));
    }

    Ok(())
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

    // First list and clean up existing subscriptions
    let existing = list_subscriptions(&credentials.id, &app_access_token).await?;
    for sub in existing.data {
        if let Ok(condition) = serde_json::from_value::<EventSubCondition>(sub.condition) {
            match condition {
                EventSubCondition::Basic {
                    broadcaster_user_id,
                }
                | EventSubCondition::ChatMessage {
                    broadcaster_user_id,
                    ..
                }
                | EventSubCondition::Follow {
                    broadcaster_user_id,
                    ..
                }
                | EventSubCondition::Subscribe {
                    broadcaster_user_id,
                }
                | EventSubCondition::ChannelPoints {
                    broadcaster_user_id,
                } if broadcaster_user_id == twitch_user_id => {
                    delete_subscription(&sub.id, &credentials.id, &app_access_token).await?;
                }
                _ => {}
            }
        }
    }

    let client = Client::new();
    let secret = TwitchCredentials::get_twitch_secret()
        .ok_or("Failed to get Twitch secret")
        .map_err(|e| WebhookError::EventSubError(e.to_string()))?;

    let mut subscription_tasks = Vec::new();

    if settings.stream_status_enabled.is_some() {
        for event_type in ["stream.online", "stream.offline"].iter() {
            subscription_tasks.push(subscribe_to_event(
                &client,
                event_type,
                "1",
                EventSubCondition::Basic {
                    broadcaster_user_id: twitch_user_id.clone(),
                },
                webhook_url,
                &secret,
                &credentials.id,
                &app_access_token,
            ));
        }
    }

    if settings.chat_messages_enabled.is_some() {
        subscription_tasks.push(subscribe_to_event(
            &client,
            "channel.chat.message",
            "1",
            EventSubCondition::ChatMessage {
                broadcaster_user_id: twitch_user_id.clone(),
                user_id: twitch_user_id.clone(),
            },
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
            EventSubCondition::ChannelPoints {
                broadcaster_user_id: twitch_user_id.clone(),
            },
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
            EventSubCondition::Follow {
                broadcaster_user_id: twitch_user_id.clone(),
                moderator_user_id: twitch_user_id.clone(),
            },
            webhook_url,
            &secret,
            &credentials.id,
            &app_access_token,
        ));

        subscription_tasks.push(subscribe_to_event(
            &client,
            "channel.subscribe",
            "1",
            EventSubCondition::Subscribe {
                broadcaster_user_id: twitch_user_id.clone(),
            },
            webhook_url,
            &secret,
            &credentials.id,
            &app_access_token,
        ));
    }

    let results = futures::future::join_all(subscription_tasks).await;

    let mut has_error = false;
    for result in results {
        if let Err(e) = result {
            has_error = true;
            tracing::error!("Failed to subscribe to event: {}", e);
        }
    }

    if has_error {
        return Err(WebhookError::EventSubError(
            "One or more subscriptions failed".to_string(),
        ));
    }

    Ok(())
}

async fn subscribe_to_event(
    client: &Client,
    event_type: &str,
    version: &str,
    condition: EventSubCondition,
    webhook_url: &str,
    secret: &str,
    client_id: &str,
    access_token: &str,
) -> Result<(), WebhookError> {
    let request = EventSubRequest {
        event_type: event_type.to_string(),
        version: version.to_string(),
        condition,
        transport: EventSubTransport {
            method: "webhook".to_string(),
            callback: webhook_url.to_string(),
            secret: secret.to_string(),
        },
    };

    tracing::debug!("Sending EventSub request: {:?}", request);

    let response = client
        .post(TWITCH_API_URL)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Client-Id", client_id)
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&request)
        .send()
        .await
        .map_err(|e| WebhookError::EventSubError(e.to_string()))?;

    if response.status().as_u16() == 409 {
        tracing::info!("Subscription already exists for event type: {}", event_type);
        return Ok(());
    }

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
