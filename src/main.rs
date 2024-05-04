mod db;

use actix_web::{
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use db::users::{init_users, User, UserError};
use env_logger::Env;
use std::collections::HashMap;

type Users = HashMap<String, User>;

async fn login(data: web::Data<Users>) -> Result<impl Responder, UserError> {
    println!("{:?}", data);
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

async fn update_site_user() -> impl Responder {
    HttpResponse::Ok().json("Update site user")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(init_users()))
            .service(
                web::scope("/auth")
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
