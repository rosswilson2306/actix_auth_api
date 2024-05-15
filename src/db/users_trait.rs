use crate::db::users::Database;
use crate::model::auth::LoginRequest;
use crate::model::user::{UpdateUserRequest, User};
use crate::{Error, Result};
use actix_web::web::Data;
use surrealdb;

// TODO: these should return Results instead of Options
pub trait UserData {
    async fn get_all_users(db: &Data<Database>) -> Result<Vec<User>>;
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn get_user(db: &Data<Database>, uuid: String) -> Result<User>;
    async fn update_user(
        db: &Data<Database>,
        uuid: String,
        user: UpdateUserRequest,
    ) -> Option<User>;
    async fn get_user_by_login(db: &Data<Database>, creds: LoginRequest) -> Result<User>;
}

impl UserData for Database {
    async fn get_all_users(db: &Data<Database>) -> Result<Vec<User>> {
        let result: std::result::Result<Vec<User>, surrealdb::Error> =
            db.client.select("users").await;

        match result {
            Ok(users) => Ok(users),
            Err(_) => Err(Error::UserNotFound),
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

    async fn get_user(db: &Data<Database>, uuid: String) -> Result<User> {
        let find_user: std::result::Result<Option<User>, surrealdb::Error> =
            db.client.select(("users", &uuid)).await;

        match find_user {
            Ok(user) => user.ok_or(Error::UserNotFound),
            Err(_) => Err(Error::UserNotFound),
        }
    }

    async fn get_user_by_login(db: &Data<Database>, creds: LoginRequest) -> Result<User> {
        // TODO: this should be done with a db query
        let users: Vec<User> = db
            .client
            .select("users")
            .await
            .map_err(|_| Error::UserNotFound)?;
        let matched_user = users
            .iter()
            .find(|user| user.email == creds.email && user.password == creds.password);

        match matched_user {
            Some(user) => Ok(user.clone()),
            None => Err(Error::UserNotFound),
        }
    }

    async fn update_user(
        db: &Data<Database>,
        uuid: String,
        user_params: UpdateUserRequest,
    ) -> Option<User> {
        let find_user: std::result::Result<Option<User>, surrealdb::Error> =
            db.client.select(("users", &uuid)).await;

        match find_user {
            Ok(_found) => match _found {
                Some(found) => {
                    let user = User {
                        uuid: String::from(&found.uuid),
                        name: user_params.name.unwrap_or(found.name),
                        email: user_params.email.unwrap_or(found.email),
                        password: user_params.password.unwrap_or(found.password),
                        role: user_params.role.unwrap_or(found.role),
                    };

                    let updated_user: std::result::Result<Option<User>, surrealdb::Error> =
                        db.client.update(("users", uuid)).merge(user).await;

                    match updated_user {
                        Ok(user) => user,
                        Err(_) => None,
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }
}
