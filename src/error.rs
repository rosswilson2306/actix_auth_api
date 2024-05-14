use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::From;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Serialize)]
pub enum Error {
    // -- user
    UserNotFound,
    LoginFailure,
    BadUserRequest,

    // TODO: move to auth::Error and derive from here
    // --auth
    AccessForbidden,
    TokenCreationFailure,
    NoAuthToken,
    InvalidAuthHeader,
    MissingSecret,
    InvalidToken,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string()) // TODO: could this be formatted to json here?
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::UserNotFound => StatusCode::NOT_FOUND,
            Error::LoginFailure => StatusCode::UNAUTHORIZED,
            Error::BadUserRequest => StatusCode::BAD_REQUEST,
            Error::AccessForbidden => StatusCode::FORBIDDEN,
            Error::TokenCreationFailure => StatusCode::BAD_REQUEST,
            Error::NoAuthToken => StatusCode::UNAUTHORIZED,
            Error::InvalidAuthHeader => StatusCode::BAD_REQUEST,
            Error::MissingSecret => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidToken => StatusCode::FORBIDDEN,
        }
    }
}
