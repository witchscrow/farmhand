use std::sync::Arc;

use crate::{jwt::decode_jwt, AppState};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{self, Response, StatusCode},
    middleware::Next,
};
use db::users::User;
use uuid::Uuid;

/// A middleware for checking the validity of the JWT token
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    // Get the auth header from the request
    let raw_auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    // Pull the full header string out of the header
    let auth_header = match raw_auth_header {
        Some(header) => {
            tracing::debug!("Auth headers found, attempting user lookup");
            header.to_str().map_err(|_| StatusCode::BAD_REQUEST)
        }
        // This middleware allows for optional users, so we just return early if no auth headers are found
        None => {
            tracing::debug!("No auth headers, skipping user lookup");
            tracing::debug!("Headers: {:#?}", req.headers());
            req.extensions_mut().insert(None::<User>);
            return Ok(next.run(req).await);
        }
    }?;
    // Full header is expected to be `Bearer token`, split by whitespace
    let mut split_header = auth_header.split_whitespace();
    // It _should_ only be two values, we care about the token value
    let (_bearer, token) = (split_header.next(), split_header.next());
    let jwt_token = token.expect("Could not parse token").to_owned();
    let token_claims = decode_jwt(jwt_token).map_err(|jwt_err| {
        tracing::error!("Error decoding jwt {jwt_err:?}");
        StatusCode::UNAUTHORIZED
    })?;
    // Convert the user id from a string to a uuid
    let user_id = Uuid::parse_str(&token_claims.claims.user_id).map_err(|e| {
        tracing::error!("Could not parse user id from token to uuid {e}");
        StatusCode::BAD_REQUEST
    })?;
    // Get the users data from the token
    let user = User::by_id(user_id, &state.db).await.map_err(|e| {
        tracing::error!("Could not get user from database in middleware {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    // Pass the user to the extensions
    tracing::trace!("Inserting user into request");
    req.extensions_mut().insert(Some(user));
    tracing::trace!("Passing to next task");
    Ok(next.run(req).await)
}
