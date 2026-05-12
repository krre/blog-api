use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use std::{
    ops::Add,
    time::{Duration, SystemTime},
};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user: User,
    exp: usize,
}

#[derive(Error, Debug)]
#[error("jsonwebtoken error: {0}")]
pub struct Error(#[from] jsonwebtoken::errors::Error);

pub fn create_token(user: User, secret: &str) -> Result<String, Error> {
    let from_now = Duration::from_hours(24 * 365 * 10); // 10 years
    let expired_future_time = SystemTime::now().add(from_now);
    let exp = OffsetDateTime::from(expired_future_time);

    let claims = Claims {
        exp: exp.unix_timestamp() as usize,
        user,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn user_id(token: &str, secret: &str) -> Result<i64, Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(data.claims.user.id)
}
