use std::collections::HashMap;
use uuid::Uuid;

pub struct User {
    id: String,
    name: String,
    email: String,
    password: String,
    role: String,
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
