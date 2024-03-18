
use actix_web::{web, App, HttpServer};
use std::io;

// Module Declarations.
mod models;
mod read_models;
mod monster_utils;
mod routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .configure(routes::config))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}