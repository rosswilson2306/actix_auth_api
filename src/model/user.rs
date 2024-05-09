use actix_web::{
    body::BoxBody, http::{header::ContentType, StatusCode}, HttpRequest, HttpResponse, Responder, ResponseError,
};
use serde::Serialize;
use derive_more::Display;

#[derive(Serialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
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
