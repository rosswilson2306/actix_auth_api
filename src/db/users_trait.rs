use crate::db::users::Database;
use crate::model::user::User;
use actix_web::web::Data;

pub trait UserData {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>>;
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User>;
}

impl UserData for Database {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>> {
        let result = db.client.select("users").await;
        match result {
            Ok(users) => Some(users),
            Err(_) => None
        }
    }

    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User> {
        let result = db.client.create(("users", new_user.id.clone())).content(new_user).await;

        match result {
            Ok(user) => user,
            Err(_) => None
        }
    }
}
