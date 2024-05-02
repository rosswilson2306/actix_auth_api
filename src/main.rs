mod db;

use actix_web::{web::{self, Json}, App, HttpServer, Responder, HttpResponse, error};
use db::users::{init_users, User};
use env_logger::Env;
use std::collections::HashMap;

type Users = HashMap<String, User>;

async fn login(data: web::Data<Users>) -> Result<impl Responder, error::Error> {
    println!("{:?}", data);
    match data.get("1") {
        Some(user) => {
            Ok(HttpResponse::Ok().json(user))
        }
        Node => {
            Err(error::ErrorNotFound("User not found"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(init_users()))
            .service(web::scope("/auth").route("/login", web::get().to(login)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
