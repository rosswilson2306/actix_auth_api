use actix_web::{web, App, HttpServer, Responder};

async fn login() -> String {
    "Log in".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/auth").route("/login", web::get().to(login)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
