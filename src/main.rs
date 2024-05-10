mod db;
mod model;

use crate::db::users::Database;
use actix_web::{
    get,
    web::{self, Data, Json},
    App, HttpResponse, HttpServer, Responder, post,
};
use db::users_trait::UserData;
use env_logger::Env;
use model::user::{User, UserError, AddUserRequest};
use validator::Validate;
use std::collections::HashMap;

type Users = HashMap<String, User>;

async fn login(data: Data<Users>) -> Result<impl Responder, UserError> {
    println!("{:?}", data);
    // TODO: use real db in order to get owned user instead of reference
    match data.get("3") {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(UserError::UserNotFound),
    }
}

async fn verify() -> impl Responder {
    HttpResponse::Ok().body("Verify token")
}

async fn update_admin_user() -> impl Responder {
    HttpResponse::Ok().json("Update admin user")
}

#[get("/users")]
async fn get_admin_users(db: Data<Database>) -> Result<Json<Vec<User>>, UserError> {
    let users = Database::get_all_users(&db).await;
    match users {
        Some(found_users) => Ok(Json(found_users)),
        None => Err(UserError::UserNotFound),
    }
}

#[post("/add-user")]
async fn add_user(body: Json<AddUserRequest>, db: Data<Database>) -> Result<Json<User>, UserError> {
    let is_valid = body.validate();
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
                web::scope("/auth")
                    .service(get_admin_users)
                    .route("/login", web::get().to(login))
                    .route("/verify", web::get().to(verify)),
            )
            .service(web::scope("/admin").route("/update", web::patch().to(update_admin_user)))
            .service(web::scope("/site").route("/update", web::patch().to(update_site_user)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
