use crate::db::{users::Database, users_trait::UserData};
use crate::model::user::{AddUserRequest, GetUserRequest, UpdateUserRequest, User, UserError};
use actix_web::{
    web::{Path, Data, Json},
    HttpResponse, Responder, get, patch, post
};
use uuid::Uuid;
use validator::Validate;

pub async fn login(_data: Data<Database>) -> impl Responder {
    HttpResponse::Ok().json("Login")
}

pub async fn verify() -> impl Responder {
    HttpResponse::Ok().body("Verify token")
}

#[get("/users")]
async fn get_users(db: Data<Database>) -> Result<Json<Vec<User>>, UserError> {
    let users = Database::get_all_users(&db).await;
    match users {
        Some(found_users) => Ok(Json(found_users)),
        None => Err(UserError::UserNotFound),
    }
}

#[get("users/{uuid}")]
async fn get_user(path: Path<GetUserRequest>, db: Data<Database>) -> Result<Json<User>, UserError> {
    let uuid = path.into_inner().uuid;
    let result = Database::get_user(&db, uuid).await;

    match result {
        Some(user) => Ok(Json(user)),
        None => Err(UserError::UserNotFound),
    }
}

#[post("/add-user")]
async fn add_user(body: Json<AddUserRequest>, db: Data<Database>) -> Result<Json<User>, UserError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let user_from_body = User {
                uuid: Uuid::new_v4().to_string(),
                name: body.name.clone(),
                email: body.email.clone(),
                password: body.password.clone(),
                role: body.role.clone(),
            };

            let new_user = Database::add_user(&db, user_from_body).await;

            match new_user {
                Some(created_user) => Ok(Json(created_user)),
                None => Err(UserError::BadUserRequest),
            }
        }
        Err(_) => Err(UserError::BadUserRequest),
    }
}

#[patch("/update-user")]
async fn update_user(
    body: Json<UpdateUserRequest>,
    db: Data<Database>,
) -> Result<Json<User>, UserError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            // TODO: allow optional fields in request and update only fields passed in request
            let user_from_body = User {
                uuid: body.uuid.clone(),
                name: body.name.clone(),
                email: body.email.clone(),
                password: body.password.clone(),
                role: body.role.clone(),
            };

            let updated_user = Database::update_user(&db, user_from_body).await;

            match updated_user {
                Some(user) => Ok(Json(user)),
                None => Err(UserError::BadUserRequest),
            }
        }
        Err(_) => Err(UserError::BadUserRequest),
    }
}

async fn update_site_user() -> impl Responder {
    HttpResponse::Ok().json("Update site user")
}
