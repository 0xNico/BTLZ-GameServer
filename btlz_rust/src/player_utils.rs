use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct Player {
    #[serde(rename = "playerId")]
    pub player_id: Pubkey,
    pub level: u8,
    pub xp: i64,
    pub chests: u64,
    #[serde(rename = "activeClass")]
    pub active_class: u64,
    #[serde(rename = "activeWeapon")]
    pub active_weapon: u64,
    pub joined: i64,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            player_id: Pubkey::new_from_array([0u8; 32]),
            level: 1,
            xp: 0,
            chests: 0,
            active_class: 0,
            active_weapon: 0,
            joined: 0,
        }
    }
}