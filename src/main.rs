mod api;
mod db;
mod model;
mod utils;

use crate::api::user::{add_user, get_user, get_users, login, update_user, verify};
use crate::db::users::Database;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use env_logger::Env;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
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
                    .service(login)
                    .route("/verify", web::get().to(verify)),
            )
            .service(get_users)
            .service(add_user)
            .service(get_user)
            .service(update_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
