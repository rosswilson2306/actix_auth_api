use crate::{Result, Error};
use crate::model::{
    auth::Claims,
    user::Role,
};
use actix_web::http::header::HeaderMap;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

const BEARER: &str = "Bearer ";

pub enum VerificationStatus {
    Granted,
    Denied,
}

pub fn create_jwt(uuid: &str, role: &Role) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uuid.to_string(),
        role: role.to_string(),
        exp: expiration as usize,
    };

    let secret = env::var("JWT_SECRET").map_err(|_| Error::TokenCreationFailure)?;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| Error::TokenCreationFailure)
}

pub fn authorize(role: Role, headers: &HeaderMap) -> Result<String> {
    match get_jwt_from_headers(headers) {
        Ok(jwt) => {
            let secret = env::var("JWT_SECRET").map_err(|_| Error::MissingSecret)?;
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            )
            .map_err(|_| Error::InvalidToken)?;

            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(Error::AccessForbidden)
            }

            // TODO: should this return a status instead
            Ok(decoded.claims.sub)
        }
        Err(e) => Err(e),
    }
}

pub fn get_jwt_from_headers(headers: &HeaderMap) -> Result<String> {
    // TODO: refactor to reduce match blocks
    let auth_result = match headers.get("authorization") {
        Some(v) => v.to_str(),
        None => return Err(Error::NoAuthToken),
    };

    let auth_header = match auth_result {
        Ok(v) => v,
        Err(_) => return Err(Error::NoAuthToken),
    };

    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeader);
    }

    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
