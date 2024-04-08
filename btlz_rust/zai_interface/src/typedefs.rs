use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PremiumItemType {
    Class,
    Weapon,
}
