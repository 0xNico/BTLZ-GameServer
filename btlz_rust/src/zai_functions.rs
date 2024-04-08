
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use zai_interface::{accounts::*, modify_player_xp_ix, ModifyPlayerXpIxArgs, ModifyPlayerXpKeys, PlayerAccount, equip_premium_item_ix, EquipPremiumItemIxArgs, EquipPremiumItemKeys, PremiumItemType};
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
use borsh::{BorshDeserialize, BorshSerialize};
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
    level: String,
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
                let player = PlayerAccount
                ::deserialize( &account.data)
                .expect("Failed to deserialize players");

                let player_data = player.0;

                PlayerJson {
                    public_key: pubkey.to_string(),
                    account: PlayerData {
                        player_id: player_data.player_id.to_string(),
                        level: player_data.level.to_string(),
                        xp: player_data.xp.to_string(),
                        chests: player_data.chests.to_string(),
                        active_class: player_data.active_class.to_string(),
                        active_weapon: player_data.active_weapon.to_string(),
                        joined: player_data.joined.to_string(),
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

pub async fn fetch_single_player(
    app_state: web::Data<AppState>,
    player_id_str: web::Path<String>,
) -> HttpResponse {
    let client = Arc::clone(&app_state.solana_client);
    let program_id = app_state.program_id;
    let player_id = Pubkey::from_str(&player_id_str).expect("Invalid player ID");

    // Derive the PDA used to store player data
    let (player_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[b"player", &player_id.to_bytes()],
        &program_id,
    );

    let result = web::block(move || client.get_account(&player_pubkey)).await;

    match result {
        Ok(Ok(account)) => {
            let player = PlayerAccount::deserialize(&account.data)
                .expect("Failed to deserialize player");

            let player_data = player.0;

            let player_json = PlayerJson {
                public_key: player_pubkey.to_string(),
                account: PlayerData {
                    player_id: player_data.player_id.to_string(),
                    level: player_data.level.to_string(),
                    xp: player_data.xp.to_string(),
                    chests: player_data.chests.to_string(),
                    active_class: player_data.active_class.to_string(),
                    active_weapon: player_data.active_weapon.to_string(),
                    joined: player_data.joined.to_string(),
                },
            };

            HttpResponse::Ok().json(player_json)
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

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ZaiInstruction {
    #[borsh]
    ModifyPlayerXp {
        xp_change: i64,
    }
}

// Handler for increasing XP
pub async fn increase_xp(
    app_state: web::Data<AppState>,
    path: web::Path<(String, i64)>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    // Log headers
    for (key, value) in req.headers() {
        log::info!("Header: {} = {:?}", key, value);
    }

    let program_id = app_state.program_id;
    let server_keypair = Arc::clone(&app_state.server_keypair);
    let client = Arc::clone(&app_state.solana_client);

    let (player_id_str, xp_change) = path.into_inner();
    
    // Execute blocking code in web::block
    let response = web::block(move || {
        let player_id = Pubkey::from_str(&player_id_str).map_err(|_| "Invalid player ID")?;

        // Derive the PDA used to store player data
        let (player_pubkey, _bump_seed) = Pubkey::find_program_address(
            &[b"player", player_id.as_ref()],
            &program_id,
        );

        let admin_pubkey = server_keypair.pubkey();
        let modify_player_xp_keys = ModifyPlayerXpKeys {
            player_account: player_pubkey,
            admin: admin_pubkey,
        };

        let modify_player_xp_args = ModifyPlayerXpIxArgs {
            xp_change,
        };

        // Create the instruction
        let instruction = modify_player_xp_ix(modify_player_xp_keys, modify_player_xp_args)
            .map_err(|_| "Failed to create instruction")?;

        // Get the latest blockhash
        let recent_blockhash = client.get_latest_blockhash()
            .map_err(|_| "Failed to get latest blockhash")?;

        // Create the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&admin_pubkey),
            &[&*server_keypair],
            recent_blockhash,
        );

        // Send the transaction
        let send_result = client.send_transaction(&transaction);

        send_result.map_err(|e| format!("Failed to send transaction: {:?}", e))
            .and_then(|signature| {
                // Instead of waiting for full confirmation, check if it's processed
                client.confirm_transaction_with_commitment(&signature, CommitmentConfig::processed())
                    .map_err(|e| format!("Failed to confirm transaction: {:?}", e))
                    .map(|_| signature)
            })
    }).await;

    // Process the response
    match response {
        Ok(Ok(signature)) => {
            let signature_str = signature.to_string();
            log::info!("Transaction successful: {}", signature_str);
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "transaction": signature_str,
            }))
        },
        Ok(Err(e)) => {
            log::error!("Transaction failed: {}", e);
            HttpResponse::InternalServerError().body(format!("Transaction failed: {}", e))
        },
        Err(e) => {
            log::error!("Server error: {:?}", e);
            HttpResponse::InternalServerError().body("Server error")
        },
    }
}

pub async fn equip_premium_item(
    app_state: web::Data<AppState>,
    path: web::Path<(String, PremiumItemType, u8)>,
) -> HttpResponse {
    let (player_id_str, item_type, item_id) = path.into_inner();
    let program_id = app_state.program_id;
    let server_keypair = Arc::clone(&app_state.server_keypair);
    let client = Arc::clone(&app_state.solana_client);

    let response = web::block(move || {
        let player_id = Pubkey::from_str(&player_id_str).map_err(|_| "Invalid player ID")?;
        let (player_pubkey, _bump_seed) = Pubkey::find_program_address(&[b"player", player_id.as_ref()], &program_id);

        let keys = EquipPremiumItemKeys {
            player_account: player_pubkey,
            admin: server_keypair.pubkey(),
        };

        let args = EquipPremiumItemIxArgs { item_type, item_id };
        let equip_instruction = equip_premium_item_ix(keys, args).map_err(|_| "Failed to create equip premium item instruction")?;

        // Set your desired compute units limit
        let compute_budget_instruction = ComputeBudgetInstruction::set_compute_unit_limit(25_000); // AVG 25,000 CU

        let transaction_instructions = vec![compute_budget_instruction, equip_instruction];

        let recent_blockhash = client.get_latest_blockhash().map_err(|_| "Failed to get latest blockhash")?;

        let transaction = Transaction::new_signed_with_payer(
            &transaction_instructions,
            Some(&keys.admin),
            &[&*server_keypair],
            recent_blockhash,
        );

        client.send_transaction(&transaction)
            .map_err(|e| format!("Failed to send transaction: {:?}", e))
            .and_then(|signature| {
                client.confirm_transaction_with_commitment(&signature, solana_sdk::commitment_config::CommitmentConfig::processed())
                    .map_err(|e| format!("Failed to confirm transaction: {:?}", e))
                    .map(|_| signature)
            })
    }).await;

    match response {
        Ok(Ok(signature)) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "transaction": signature.to_string(),
        })),
        Ok(Err(e)) => HttpResponse::InternalServerError().body(format!("Transaction failed: {}", e)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Server error: {:?}", e)),
    }
}
