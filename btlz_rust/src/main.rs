use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io;
use std::net::TcpListener;
use crate::models::ActiveBattles;
use crate::routes::config;
use tokio::signal;
use tokio::time::{self, Duration};
use env_logger::Env;

// Module Declarations.
mod models;
mod read_models;
mod monster_utils;
mod routes;

async fn run_server(active_battles_data: web::Data<ActiveBattles>) -> io::Result<Server> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    let server = HttpServer::new(move || {
        App::new()
            .app_data(active_battles_data.clone())
            .configure(config)
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let active_battles_data = web::Data::new(ActiveBattles {
        battles: Mutex::new(HashMap::new()),
    });

    let server = run_server(active_battles_data).await?;

    println!("BTLZ - Server has started running on PORT: 3000");

    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        println!("BTLZ - Server is closing down in 10 seconds.");
        time::sleep(Duration::from_secs(10)).await;
    };

    tokio::select! {
        _ = server => println!("Server stopped."),
        _ = ctrl_c => {},
    }

    Ok(())
}