use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use std::collections::HashMap;
use uuid::Uuid;

use crate::model::user::User;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0,0.1:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        client.use_ns("surreal").use_db("users").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("users")
        })
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
