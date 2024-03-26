use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct Player {
    pub player_id: Pubkey,
    pub xp: i64,
    pub chests: u64,
    pub active_class: u64,
    pub active_weapon: u64,
    pub joined: i64,
}
