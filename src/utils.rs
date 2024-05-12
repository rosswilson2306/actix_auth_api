use crate::model::{
    auth::{Claims, JWTError},
    user::Role,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, EncodingKey, Header};
use std::env;

pub fn create_jwt(uuid: &str, role: &Role) -> Result<String, JWTError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uuid.to_string(),
        role: role.to_string(),
        exp: expiration as usize,
    };

    let secret = env::var("JWT_SECRET").map_err(|_| JWTError::CreationFailure)?;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).map_err(|_| JWTError::CreationFailure)
}
