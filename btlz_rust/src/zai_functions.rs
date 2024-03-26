use borsh::BorshDeserialize;
use actix_web::{web, HttpResponse, error::BlockingError};
use actix_web::web::block;
use solana_sdk::pubkey::Pubkey;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::AppState;
use crate::player_utils::Player;
use std::time::Instant;
use std::str::FromStr;
use std::sync::Arc;

// Serializable version of the Player struct for HTTP responses
#[derive(Serialize, Deserialize)]
struct PlayerResponse {
    player_id: String,
    xp: i64,
    chests: u64,
    active_class: u64,
    active_weapon: u64,
    joined: i64,
}

impl PlayerResponse {
    // Function to create a new PlayerResponse from the on-chain Player data
    fn from_onchain_data(player_id: &Pubkey, player_data: &Player) -> Self {
        PlayerResponse {
            player_id: player_id.to_string(),
            xp: player_data.xp,
            chests: player_data.chests,
            active_class: player_data.active_class,
            active_weapon: player_data.active_weapon,
            joined: player_data.joined,
        }
    }
}

// Updated response structure to include response time
#[derive(Serialize)]
struct RpcConnectionResponse {
    connected: bool,
    message: String,
    response_time_ms: u128, // Response time in milliseconds
}

// Function to test RPC connection and report back with response time
pub async fn test_rpc_connection(app_state: web::Data<AppState>) -> HttpResponse {
    let rpc_client = app_state.solana_client.clone();

    // Start measuring time
    let start = Instant::now();

    // Use web::block to offload the blocking operation
    let result = web::block(move || rpc_client.get_version()).await;

    // Stop measuring time
    let duration = start.elapsed();

    match result {
        Ok(version_info) => HttpResponse::Ok().json(RpcConnectionResponse {
            connected: true,
            message: format!("Connected to Solana DevNet. Version: {:?}", version_info),
            response_time_ms: duration.as_millis(), // Report response time
        }),
        Err(e) => HttpResponse::InternalServerError().json(RpcConnectionResponse {
            connected: false,
            message: format!("Failed to connect to Solana DevNet. Error: {:?}", e),
            response_time_ms: duration.as_millis(), // Report response time even in case of failure
        }),
    }
}

pub async fn fetch_players(app_state: web::Data<AppState>) -> HttpResponse {
    let client = Arc::clone(&app_state.solana_client);
    let program_id = app_state.program_id;

    // Using web::block to offload the synchronous operation to a threadpool
    let accounts_result = web::block(move || {
        client.get_program_accounts(&program_id)
    }).await;

    match accounts_result {
        Ok(Ok(accounts)) => {
            // Process accounts
            // Convert accounts to your desired format here
            HttpResponse::Ok().json(accounts) // Placeholder response
        },
        Ok(Err(e)) => {
            // Handle RPC client error
            eprintln!("RPC error: {:?}", e);
            HttpResponse::InternalServerError().json("RPC error occurred")
        },
        Err(e) => {
            // Handle blocking operation error
            eprintln!("Blocking operation error: {:?}", e);
            HttpResponse::InternalServerError().json("Server error occurred")
        }
    }
}