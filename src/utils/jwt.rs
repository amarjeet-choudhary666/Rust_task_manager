use chrono::{Utc, Duration};
use jsonwebtoken::{encode, EncodingKey, Header, errors::Result, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub token_type: String,
}

lazy_static::lazy_static! {
    static ref ACCESS_TOKEN_SECRET: String = std::env::var("ACCESS_TOKEN_SECRET")
        .expect("❌ failed to get ACCESS_TOKEN_SECRET from .env");
    static ref REFRESH_TOKEN_SECRET: String = std::env::var("REFRESH_TOKEN_SECRET")
        .expect("❌ failed to get REFRESH_TOKEN_SECRET from .env");
}

pub fn create_access_token(user_id: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15)) 
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        token_type: "access".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(ACCESS_TOKEN_SECRET.as_ref()),
    )
}

pub fn create_refresh_token(user_id: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        token_type: "refresh".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(REFRESH_TOKEN_SECRET.as_ref()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(ACCESS_TOKEN_SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

