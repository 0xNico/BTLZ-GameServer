use borsh::BorshDeserialize;
use zai_interface::{PlayerAccount, accounts::*};
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
use std::borrow::Borrow;

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

#[derive(Serialize)]
struct PlayerJson {
    public_key: String,
    account: PlayerData,
}

#[derive(Serialize)]
struct PlayerData {
    player_id: String,
    xp: String,
    chests: String,
    active_class: String,
    active_weapon: String,
    joined: String,
}

pub async fn fetch_players(app_state: web::Data<AppState>) -> HttpResponse {
    let client = Arc::clone(&app_state.solana_client);
    let program_id = app_state.program_id;

    let result = web::block(move || client.get_program_accounts(&program_id)).await;

    match result {
        Ok(Ok(accounts)) => {
            let players: Vec<PlayerJson> = accounts.into_iter().map(|(pubkey, account)| {
                let player = Player::try_from_slice(&account.data)
                    .unwrap_or_else(|e| {
                        log::error!("Failed to deserialize Player: {:?}", e);
                        Player::default()
                    });


                PlayerJson {
                    public_key: pubkey.to_string(),
                    account: PlayerData {
                        player_id: player.player_id.to_string(),
                        xp: player.xp.to_string(),
                        chests: player.chests.to_string(),
                        active_class: player.active_class.to_string(),
                        active_weapon: player.active_weapon.to_string(),
                        joined: player.joined.to_string(),
                    },
                }
            }).collect();

            HttpResponse::Ok().json(players)
        },
        Ok(Err(e)) => {
            log::error!("RPC error: {:?}", e);
            HttpResponse::InternalServerError().json("RPC error occurred")
        },
        Err(e) => {
            log::error!("Blocking operation error: {:?}", e);
            HttpResponse::InternalServerError().json("Server error occurred")
        },
    }
}