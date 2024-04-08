//zai_interface/src/errors.rs
use anchor_lang::prelude::thiserror;
use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum ZaiError {
    #[error("Invalid class selected. Valid options are 101, 102, or 103.")]
    InvalidClass = 6000,
    #[error("default active weapon must match the active class.")]
    WeaponClassMismatch = 6001,
    #[error("Changing to the same class is not allowed.")]
    ClassChangeToSameNotAllowed = 6002,
    #[error("Unauthorized attempt to modify player account - Server did not sign.")]
    Unauthorized = 6003,
    #[error("XP modification resulted in overflow.")]
    XpOverflow = 6004,
    #[error("Player has reached the level cap.")]
    LevelCapReached = 6005,
    #[error("Player does not have enough XP to level up.")]
    NotEnoughXp = 6006,
    #[error("Attempted to switch to default class.")]
    InvalidPremiumClass = 6007,
    #[error("Attempted to switch to default weapon.")]
    InvalidPremiumWeapon = 6008,
}
impl From<ZaiError> for ProgramError {
    fn from(e: ZaiError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for ZaiError {
    fn type_of() -> &'static str {
        "ZaiError"
    }
}
impl PrintProgramError for ZaiError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
