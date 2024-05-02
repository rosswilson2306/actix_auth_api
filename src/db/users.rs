use std::collections::HashMap;
use actix_web::{Responder, body::BoxBody, HttpResponse, http::header::ContentType, HttpRequest};
use serde::{Serialize};
use uuid::Uuid;

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

pub fn init_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        String::from("1"),
        User {
            id: Uuid::new_v4().to_string(),
            name: String::from("User 1"),
            email: String::from("user1@user.com"),
            password: String::from("1234"),
            role: String::from("User"),
        },
    );
    users.insert(
        String::from("2"),
        User {
            id: Uuid::new_v4().to_string(),
            name: String::from("User 2"),
            email: String::from("user2@user.com"),
            password: String::from("4321"),
            role: String::from("Admin"),
        },
    );

    users
}
