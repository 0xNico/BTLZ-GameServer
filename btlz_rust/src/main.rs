
use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::{ActiveBattles, BattleInstance};
use env_logger::Env;

// Module Declarations.
mod models;
mod read_models;
mod monster_utils;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let active_battles_data = web::Data::new(ActiveBattles {
        battles: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(active_battles_data.clone())
            .configure(routes::config) // Ensure this points to your routes configuration.
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}