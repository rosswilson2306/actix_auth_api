use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub role: String,
    pub sub: String,
    pub exp: usize,
}

// #[derive(Debug)]
// pub enum Error {
//     TokenCreationFailure,
//     NoAuthToken,
//     InvalidAuthHeader,
//     MissingSecret,
//     InvalidToken,
// }

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
