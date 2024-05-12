use crate::db::users::Database;
use crate::model::auth::LoginRequest;
use crate::model::user::{UpdateUserRequest, User};
use actix_web::web::Data;
use surrealdb::Error;

// TODO: these should return Results instead of Options
pub trait UserData {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>>;
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn get_user(db: &Data<Database>, uuid: String) -> Option<User>;
    async fn update_user(
        db: &Data<Database>,
        uuid: String,
        user: UpdateUserRequest,
    ) -> Option<User>;
    async fn get_user_by_login(db: &Data<Database>, creds: LoginRequest) -> Option<User>;
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

    async fn get_user_by_login(db: &Data<Database>, creds: LoginRequest) -> Option<User> {
        // TODO: explore using query to select user with the correct fields as select isn't working
        // with uuid
        // TODO: password validation
        let find_user: Result<Option<User>, Error> = db.client.select(("users", creds.email)).await;

        match find_user {
            Ok(user) => user,
            Err(_) => None,
        }
    }

    async fn update_user(
        db: &Data<Database>,
        uuid: String,
        user_params: UpdateUserRequest,
    ) -> Option<User> {
        let find_user: Result<Option<User>, Error> = db.client.select(("users", &uuid)).await;

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

                    let updated_user: Result<Option<User>, Error> =
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
