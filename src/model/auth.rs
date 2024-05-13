use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub role: String,
    pub sub: String,
    pub exp: usize,
}

pub enum JWTError {
    CreationFailure,
    VerifyFailure,
    InvalidToken,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
