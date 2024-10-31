use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum JWTError {
    DecodingError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,      // Expiry time of the token
    pub iat: usize,      // Issued at time of the token
    pub user_id: String, // User ID associated with the token
}

/// Gets the JWT_SECRET from the environment variables and converts to bytes
fn get_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set")
        .into_bytes()
}

/// Creates a JWT token containing the given user_id
pub fn encode_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = get_secret();
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims {
        iat,
        exp,
        user_id: user_id.to_owned(),
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(&jwt_secret),
    )
}

/// Decodes a JWT token
pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, JWTError> {
    let jwt_secret = get_secret();
    decode(
        &jwt_token,
        &DecodingKey::from_secret(&jwt_secret),
        &Validation::default(),
    )
    .map_err(|_| JWTError::DecodingError)
}
