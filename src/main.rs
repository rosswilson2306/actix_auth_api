mod db;
mod model;

use crate::db::users::Database;
use actix_web::{
    get, patch, post,
    web::{self, Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use db::users_trait::UserData;
use env_logger::Env;
use model::user::{AddUserRequest, GetUserRequest, UpdateUserRequest, User, UserError};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

type Users = HashMap<String, User>;

async fn login(data: Data<Users>) -> impl Responder {
    HttpResponse::Ok().json("Login")
}

async fn verify() -> impl Responder {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let db = Database::init()
        .await
        .expect("Error connecting to the database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone()) // TODO: Should this use an Arc?
            .service(
                web::scope("/all") // TODO: Fix
                    .service(get_users)
                    .service(add_user)
                    .service(get_user)
                    .service(update_user),
            )
            .service(
                web::scope("/auth")
                    .route("/login", web::get().to(login))
                    .route("/verify", web::get().to(verify)),
            )
            .service(web::scope("/site").route("/update", web::patch().to(update_site_user)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
