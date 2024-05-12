use actix_web::{
    body::BoxBody, http::{header::ContentType, StatusCode}, HttpRequest, HttpResponse, Responder, ResponseError,
};
use serde::{Deserialize, Serialize};
use derive_more::Display;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

pub enum Role {
    Admim,
    Site,
}

impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Debug, Display)]
pub enum UserError {
    UserNotFound,
    BadUserRequest,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::BadUserRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[derive(Validate, Serialize, Deserialize)]
pub struct AddUserRequest {
    #[validate(length(min = 1, message = "name required"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "password required"))]
    pub password: String,
    #[validate(length(min = 1, message = "confirmation password required"))] // TODO: validate that
    #[validate(must_match = "password")]
    // passowrds match
    pub confirm_password: String,
    // TODO: one of the two roles
    pub role: String,
}
