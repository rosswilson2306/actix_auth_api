use crate::db::users::Database;
use crate::model::user::User;
use actix_web::web::Data;

pub trait UserData {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>>;
}

impl UserData for Database {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>> {
        let result = db.client.select("users").await;
        match result {
            Ok(users) => Some(users),
            Err(_) => None
        }
    }
}
