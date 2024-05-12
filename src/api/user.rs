use crate::db::{users::Database, users_trait::UserData};
use crate::model::auth::{LoginRequest, LoginResponse};
use crate::model::user::{
    AddUserRequest, GetUserRequest, Role, UpdateUserRequest, User, UserError,
};
use crate::utils::create_jwt;
use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;
use validator::Validate;

#[post("/login")]
async fn login(
    db: Data<Database>,
    body: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, UserError> {
    let user = Database::get_user_by_login(&db, body.clone()).await;

    match user {
        Ok(found_user) => {
            // TODO: role from user
            let token =
                create_jwt(&found_user.uuid, &Role::Admin).map_err(|_| UserError::LoginFailure)?;
            Ok(Json(LoginResponse { token }))
        }
        Err(_) => Err(UserError::UserNotFound),
    }
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

#[patch("/update-user/{uuid}")]
async fn update_user(
    body: Json<UpdateUserRequest>,
    path: Path<String>,
    db: Data<Database>,
) -> Result<Json<User>, UserError> {
    let uuid = path.into_inner();
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            // TODO: allow optional fields in request and update only fields passed in request
            let updated_user = Database::update_user(&db, uuid, body.into_inner()).await;

            match updated_user {
                Some(user) => Ok(Json(user)),
                None => Err(UserError::BadUserRequest),
            }
        }
        Err(_) => Err(UserError::BadUserRequest),
    }
}
