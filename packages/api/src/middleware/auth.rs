use crate::jwt::decode_jwt;
use axum::{
    body::Body,
    extract::Request,
    http::{self, Response, StatusCode},
    middleware::Next,
};

/// A middleware for checking the validity of the JWT token
pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response<Body>, StatusCode> {
    // Get the auth header from the request
    let raw_auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    // Pull the full header string out of the header
    let auth_header = match raw_auth_header {
        Some(header) => header.to_str().map_err(|_| StatusCode::BAD_REQUEST),
        None => return Err(StatusCode::BAD_REQUEST),
    }?;
    // Full header is expected to be `Bearer token`, split by whitespace
    let mut split_header = auth_header.split_whitespace();
    // It _should_ only be two values, we care about the token value
    let (_bearer, token) = (split_header.next(), split_header.next());
    let jwt_token = token.expect("Could not parse token").to_owned();
    let token_claims = match decode_jwt(jwt_token) {
        Ok(token) => token,
        Err(jwt_err) => return Err(StatusCode::UNAUTHORIZED),
    };
    // TODO: Using the token_claims, get the User and add it to the request for downstream usage
    Ok(next.run(req).await)
}
