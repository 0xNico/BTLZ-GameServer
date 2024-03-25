// src/app_state.rs
use std::sync::Arc;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use crate::models::ActiveBattles;


pub struct AppState {
    pub solana_client: Arc<RpcClient>,
    pub server_keypair: Arc<solana_sdk::signature::Keypair>,
    pub active_battles: ActiveBattles,
    pub program_id: Pubkey,
}

