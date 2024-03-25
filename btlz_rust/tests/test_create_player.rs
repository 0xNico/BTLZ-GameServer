use actix_web::{web, App, test, http::StatusCode};
use btlz_rust::{models::ActiveBattles, routes::config, AppState};
use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signer}, transaction::Transaction
};
use std::{sync::{Arc, Mutex}, collections::HashMap, str::FromStr};

#[tokio::test]
async fn test_create_player_endpoint() {
    let server_keypair_path = "cfg/zaigDCJxJdzSh6ETPosxWfVrzemjLnMJxxqhZCfdEqU.json";
    let server_keypair = Arc::new(read_keypair_file(server_keypair_path).expect("Failed to read server keypair"));
    let rpc_url = "https://api.devnet.solana.com";
    let solana_client = Arc::new(RpcClient::new(rpc_url));
    let program_id = Pubkey::from_str("HWjAY4TNEiAQquRKmwRXMabXf1PMGp36QyQgA162XdNr").unwrap();

    let active_battles = ActiveBattles {
        battles: Mutex::new(HashMap::new()),
    };

    let app_state = web::Data::new(AppState {
        solana_client: solana_client.clone(),
        server_keypair,
        active_battles,
        program_id,
    });

    let app = test::init_service(
        App::new()
            .app_data(app_state)
            .configure(config)
    ).await;

    let test_user_keypair = Keypair::new();
    let test_user_pubkey = test_user_keypair.pubkey().to_string();
    let airdrop_amount = 500_000_000; // 0.5 SOL

    let airdrop_signature = solana_client
        .request_airdrop(&test_user_keypair.pubkey(), airdrop_amount)
        .expect("Failed to request airdrop");

    // Confirm the airdrop transaction
    solana_client
        .confirm_transaction_with_commitment(&airdrop_signature, CommitmentConfig::confirmed())
        .expect("Failed to confirm airdrop transaction");

    let req_body = json!({
        "signer_pubkey": test_user_pubkey,
        "active_class": 101,
        "active_weapon": 101,
    });

    let req = test::TestRequest::post()
        .uri("/create_player")
        .set_json(&req_body) // Directly using the serde_json::Value here
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
