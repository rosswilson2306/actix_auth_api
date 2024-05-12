use crate::db::users::Database;
use crate::model::user::User;
use actix_web::web::Data;
use surrealdb::Error;

pub trait UserData {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>>;
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn get_user(db: &Data<Database>, uuid: String) -> Option<User>;
}

impl UserData for Database {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>> {
        let result = db.client.select("users").await;
        match result {
            Ok(users) => Some(users),
            Err(_) => None,
        }
    }

    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User> {
        let result = db
            .client
            .create(("users", new_user.uuid.clone()))
            .content(new_user)
            .await;

        match result {
            Ok(user) => user,
            Err(_) => None,
        }
    }

    async fn get_user(db: &Data<Database>, uuid: String) -> Option<User> {
        let find_user: Result<Option<User>, Error> = db.client.select(("users", &uuid)).await;

        match find_user {
            Ok(user) => user,
            Err(_) => None,
        }
    }
}
