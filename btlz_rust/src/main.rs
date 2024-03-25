//main.rs
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io;
use std::net::TcpListener;
use std::str::FromStr;
use crate::models::ActiveBattles;
use crate::routes::config;
use crate::app_state::AppState;
use tokio::signal;
use tokio::time::{self, Duration};
use env_logger::Env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::pubkey::Pubkey;


mod models;
mod read_models;
mod monster_utils;
mod routes;
mod zai_functions;
mod app_state;

async fn run_server(app_state: web::Data<AppState>) -> io::Result<Server> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(config)
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let server_keypair_path = "cfg/zaigDCJxJdzSh6ETPosxWfVrzemjLnMJxxqhZCfdEqU.json";
    let server_keypair = Arc::new(read_keypair_file(server_keypair_path).expect("Failed to read server keypair"));
    let rpc_url = "https://api.devnet.solana.com"; // Change to your Solana cluster URL
    let solana_client = Arc::new(RpcClient::new(rpc_url));
    let program_id = Pubkey::from_str("HWjAY4TNEiAQquRKmwRXMabXf1PMGp36QyQgA162XdNr").unwrap();

    let active_battles = ActiveBattles {
        battles: Mutex::new(HashMap::new()),
    };

    let app_state = web::Data::new(AppState {
        solana_client,
        server_keypair,
        active_battles,
        program_id,
    });

    let server = run_server(app_state).await?;

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
