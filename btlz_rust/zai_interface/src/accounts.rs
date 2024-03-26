//zai_interface/src/accounts.rs
use std::io::{Read, Write};
use ::borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const PLAYER_ACCOUNT_DISCM: [u8; 8] = [205, 222, 112, 7, 165, 155, 206, 218];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Player {
    pub player_id: Pubkey,
    pub xp: i64,
    pub chests: u64,
    pub active_class: u64,
    pub active_weapon: u64,
    pub joined: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerAccount(pub Player);
impl PlayerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLAYER_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PLAYER_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Player::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLAYER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
