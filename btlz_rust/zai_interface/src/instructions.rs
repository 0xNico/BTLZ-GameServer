use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey, program_error::ProgramError,
};
use std::io::Read;
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum ZaiProgramIx {
    CreatePlayer(CreatePlayerIxArgs),
    ChangeDefaultClass(ChangeDefaultClassIxArgs),
    ModifyPlayerXp(ModifyPlayerXpIxArgs),
    IncreasePlayerLevel,
    EquipPremiumItem(EquipPremiumItemIxArgs),
}
impl ZaiProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            CREATE_PLAYER_IX_DISCM => {
                Ok(Self::CreatePlayer(CreatePlayerIxArgs::deserialize(&mut reader)?))
            }
            CHANGE_DEFAULT_CLASS_IX_DISCM => {
                Ok(
                    Self::ChangeDefaultClass(
                        ChangeDefaultClassIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            MODIFY_PLAYER_XP_IX_DISCM => {
                Ok(Self::ModifyPlayerXp(ModifyPlayerXpIxArgs::deserialize(&mut reader)?))
            }
            INCREASE_PLAYER_LEVEL_IX_DISCM => Ok(Self::IncreasePlayerLevel),
            EQUIP_PREMIUM_ITEM_IX_DISCM => {
                Ok(
                    Self::EquipPremiumItem(
                        EquipPremiumItemIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            _ => {
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("discm {:?} not found", maybe_discm),
                    ),
                )
            }
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreatePlayer(args) => {
                writer.write_all(&CREATE_PLAYER_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ChangeDefaultClass(args) => {
                writer.write_all(&CHANGE_DEFAULT_CLASS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ModifyPlayerXp(args) => {
                writer.write_all(&MODIFY_PLAYER_XP_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::IncreasePlayerLevel => {
                writer.write_all(&INCREASE_PLAYER_LEVEL_IX_DISCM)
            }
            Self::EquipPremiumItem(args) => {
                writer.write_all(&EQUIP_PREMIUM_ITEM_IX_DISCM)?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const CREATE_PLAYER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreatePlayerAccounts<'me, 'info> {
    pub player_account: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePlayerKeys {
    pub player_account: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePlayerAccounts<'_, '_>> for CreatePlayerKeys {
    fn from(accounts: CreatePlayerAccounts) -> Self {
        Self {
            player_account: *accounts.player_account.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePlayerKeys> for [AccountMeta; CREATE_PLAYER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePlayerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.player_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_PLAYER_IX_ACCOUNTS_LEN]> for CreatePlayerKeys {
    fn from(pubkeys: [Pubkey; CREATE_PLAYER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            player_account: pubkeys[0],
            signer: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreatePlayerAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PLAYER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePlayerAccounts<'_, 'info>) -> Self {
        [
            accounts.player_account.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PLAYER_IX_ACCOUNTS_LEN]>
for CreatePlayerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_PLAYER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            player_account: &arr[0],
            signer: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_PLAYER_IX_DISCM: [u8; 8] = [19, 178, 189, 216, 159, 134, 0, 192];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePlayerIxArgs {
    pub active_class: u8,
    pub active_weapon: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePlayerIxData(pub CreatePlayerIxArgs);
impl From<CreatePlayerIxArgs> for CreatePlayerIxData {
    fn from(args: CreatePlayerIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePlayerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PLAYER_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_PLAYER_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreatePlayerIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PLAYER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_player_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePlayerKeys,
    args: CreatePlayerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PLAYER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePlayerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_player_ix(
    keys: CreatePlayerKeys,
    args: CreatePlayerIxArgs,
) -> std::io::Result<Instruction> {
    create_player_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_player_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlayerAccounts<'_, '_>,
    args: CreatePlayerIxArgs,
) -> ProgramResult {
    let keys: CreatePlayerKeys = accounts.into();
    let ix = create_player_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_player_invoke(
    accounts: CreatePlayerAccounts<'_, '_>,
    args: CreatePlayerIxArgs,
) -> ProgramResult {
    create_player_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_player_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlayerAccounts<'_, '_>,
    args: CreatePlayerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePlayerKeys = accounts.into();
    let ix = create_player_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_player_invoke_signed(
    accounts: CreatePlayerAccounts<'_, '_>,
    args: CreatePlayerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_player_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_player_verify_account_keys(
    accounts: CreatePlayerAccounts<'_, '_>,
    keys: CreatePlayerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.player_account.key, keys.player_account),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_player_verify_writable_privileges<'me, 'info>(
    accounts: CreatePlayerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.player_account, accounts.signer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_player_verify_signer_privileges<'me, 'info>(
    accounts: CreatePlayerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_player_verify_account_privileges<'me, 'info>(
    accounts: CreatePlayerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_player_verify_writable_privileges(accounts)?;
    create_player_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_DEFAULT_CLASS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ChangeDefaultClassAccounts<'me, 'info> {
    pub player_account: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangeDefaultClassKeys {
    pub player_account: Pubkey,
    pub signer: Pubkey,
}
impl From<ChangeDefaultClassAccounts<'_, '_>> for ChangeDefaultClassKeys {
    fn from(accounts: ChangeDefaultClassAccounts) -> Self {
        Self {
            player_account: *accounts.player_account.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<ChangeDefaultClassKeys>
for [AccountMeta; CHANGE_DEFAULT_CLASS_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeDefaultClassKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.player_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CHANGE_DEFAULT_CLASS_IX_ACCOUNTS_LEN]> for ChangeDefaultClassKeys {
    fn from(pubkeys: [Pubkey; CHANGE_DEFAULT_CLASS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            player_account: pubkeys[0],
            signer: pubkeys[1],
        }
    }
}
impl<'info> From<ChangeDefaultClassAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_DEFAULT_CLASS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangeDefaultClassAccounts<'_, 'info>) -> Self {
        [accounts.player_account.clone(), accounts.signer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_DEFAULT_CLASS_IX_ACCOUNTS_LEN]>
for ChangeDefaultClassAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CHANGE_DEFAULT_CLASS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            player_account: &arr[0],
            signer: &arr[1],
        }
    }
}
pub const CHANGE_DEFAULT_CLASS_IX_DISCM: [u8; 8] = [
    172,
    236,
    215,
    173,
    37,
    79,
    103,
    124,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeDefaultClassIxArgs {
    pub new_class: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeDefaultClassIxData(pub ChangeDefaultClassIxArgs);
impl From<ChangeDefaultClassIxArgs> for ChangeDefaultClassIxData {
    fn from(args: ChangeDefaultClassIxArgs) -> Self {
        Self(args)
    }
}
impl ChangeDefaultClassIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_DEFAULT_CLASS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CHANGE_DEFAULT_CLASS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ChangeDefaultClassIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_DEFAULT_CLASS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_default_class_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeDefaultClassKeys,
    args: ChangeDefaultClassIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_DEFAULT_CLASS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChangeDefaultClassIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_default_class_ix(
    keys: ChangeDefaultClassKeys,
    args: ChangeDefaultClassIxArgs,
) -> std::io::Result<Instruction> {
    change_default_class_ix_with_program_id(crate::ID, keys, args)
}
pub fn change_default_class_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeDefaultClassAccounts<'_, '_>,
    args: ChangeDefaultClassIxArgs,
) -> ProgramResult {
    let keys: ChangeDefaultClassKeys = accounts.into();
    let ix = change_default_class_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_default_class_invoke(
    accounts: ChangeDefaultClassAccounts<'_, '_>,
    args: ChangeDefaultClassIxArgs,
) -> ProgramResult {
    change_default_class_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn change_default_class_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeDefaultClassAccounts<'_, '_>,
    args: ChangeDefaultClassIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeDefaultClassKeys = accounts.into();
    let ix = change_default_class_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_default_class_invoke_signed(
    accounts: ChangeDefaultClassAccounts<'_, '_>,
    args: ChangeDefaultClassIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_default_class_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn change_default_class_verify_account_keys(
    accounts: ChangeDefaultClassAccounts<'_, '_>,
    keys: ChangeDefaultClassKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.player_account.key, keys.player_account),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn change_default_class_verify_writable_privileges<'me, 'info>(
    accounts: ChangeDefaultClassAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.player_account, accounts.signer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_default_class_verify_signer_privileges<'me, 'info>(
    accounts: ChangeDefaultClassAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_default_class_verify_account_privileges<'me, 'info>(
    accounts: ChangeDefaultClassAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_default_class_verify_writable_privileges(accounts)?;
    change_default_class_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MODIFY_PLAYER_XP_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ModifyPlayerXpAccounts<'me, 'info> {
    pub player_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ModifyPlayerXpKeys {
    pub player_account: Pubkey,
    pub admin: Pubkey,
}
impl From<ModifyPlayerXpAccounts<'_, '_>> for ModifyPlayerXpKeys {
    fn from(accounts: ModifyPlayerXpAccounts) -> Self {
        Self {
            player_account: *accounts.player_account.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<ModifyPlayerXpKeys> for [AccountMeta; MODIFY_PLAYER_XP_IX_ACCOUNTS_LEN] {
    fn from(keys: ModifyPlayerXpKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.player_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MODIFY_PLAYER_XP_IX_ACCOUNTS_LEN]> for ModifyPlayerXpKeys {
    fn from(pubkeys: [Pubkey; MODIFY_PLAYER_XP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            player_account: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<ModifyPlayerXpAccounts<'_, 'info>>
for [AccountInfo<'info>; MODIFY_PLAYER_XP_IX_ACCOUNTS_LEN] {
    fn from(accounts: ModifyPlayerXpAccounts<'_, 'info>) -> Self {
        [accounts.player_account.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MODIFY_PLAYER_XP_IX_ACCOUNTS_LEN]>
for ModifyPlayerXpAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MODIFY_PLAYER_XP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            player_account: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const MODIFY_PLAYER_XP_IX_DISCM: [u8; 8] = [39, 15, 12, 126, 234, 101, 181, 20];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyPlayerXpIxArgs {
    pub xp_change: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModifyPlayerXpIxData(pub ModifyPlayerXpIxArgs);
impl From<ModifyPlayerXpIxArgs> for ModifyPlayerXpIxData {
    fn from(args: ModifyPlayerXpIxArgs) -> Self {
        Self(args)
    }
}
impl ModifyPlayerXpIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODIFY_PLAYER_XP_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        MODIFY_PLAYER_XP_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ModifyPlayerXpIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODIFY_PLAYER_XP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn modify_player_xp_ix_with_program_id(
    program_id: Pubkey,
    keys: ModifyPlayerXpKeys,
    args: ModifyPlayerXpIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MODIFY_PLAYER_XP_IX_ACCOUNTS_LEN] = keys.into();
    let data: ModifyPlayerXpIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn modify_player_xp_ix(
    keys: ModifyPlayerXpKeys,
    args: ModifyPlayerXpIxArgs,
) -> std::io::Result<Instruction> {
    modify_player_xp_ix_with_program_id(crate::ID, keys, args)
}
pub fn modify_player_xp_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ModifyPlayerXpAccounts<'_, '_>,
    args: ModifyPlayerXpIxArgs,
) -> ProgramResult {
    let keys: ModifyPlayerXpKeys = accounts.into();
    let ix = modify_player_xp_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn modify_player_xp_invoke(
    accounts: ModifyPlayerXpAccounts<'_, '_>,
    args: ModifyPlayerXpIxArgs,
) -> ProgramResult {
    modify_player_xp_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn modify_player_xp_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ModifyPlayerXpAccounts<'_, '_>,
    args: ModifyPlayerXpIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ModifyPlayerXpKeys = accounts.into();
    let ix = modify_player_xp_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn modify_player_xp_invoke_signed(
    accounts: ModifyPlayerXpAccounts<'_, '_>,
    args: ModifyPlayerXpIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    modify_player_xp_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn modify_player_xp_verify_account_keys(
    accounts: ModifyPlayerXpAccounts<'_, '_>,
    keys: ModifyPlayerXpKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.player_account.key, keys.player_account),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn modify_player_xp_verify_writable_privileges<'me, 'info>(
    accounts: ModifyPlayerXpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.player_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn modify_player_xp_verify_signer_privileges<'me, 'info>(
    accounts: ModifyPlayerXpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn modify_player_xp_verify_account_privileges<'me, 'info>(
    accounts: ModifyPlayerXpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    modify_player_xp_verify_writable_privileges(accounts)?;
    modify_player_xp_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_PLAYER_LEVEL_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct IncreasePlayerLevelAccounts<'me, 'info> {
    pub player_account: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreasePlayerLevelKeys {
    pub player_account: Pubkey,
    pub signer: Pubkey,
}
impl From<IncreasePlayerLevelAccounts<'_, '_>> for IncreasePlayerLevelKeys {
    fn from(accounts: IncreasePlayerLevelAccounts) -> Self {
        Self {
            player_account: *accounts.player_account.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<IncreasePlayerLevelKeys>
for [AccountMeta; INCREASE_PLAYER_LEVEL_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreasePlayerLevelKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.player_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; INCREASE_PLAYER_LEVEL_IX_ACCOUNTS_LEN]> for IncreasePlayerLevelKeys {
    fn from(pubkeys: [Pubkey; INCREASE_PLAYER_LEVEL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            player_account: pubkeys[0],
            signer: pubkeys[1],
        }
    }
}
impl<'info> From<IncreasePlayerLevelAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_PLAYER_LEVEL_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreasePlayerLevelAccounts<'_, 'info>) -> Self {
        [accounts.player_account.clone(), accounts.signer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_PLAYER_LEVEL_IX_ACCOUNTS_LEN]>
for IncreasePlayerLevelAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INCREASE_PLAYER_LEVEL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            player_account: &arr[0],
            signer: &arr[1],
        }
    }
}
pub const INCREASE_PLAYER_LEVEL_IX_DISCM: [u8; 8] = [
    218,
    239,
    175,
    171,
    178,
    245,
    58,
    224,
];
#[derive(Clone, Debug, PartialEq)]
pub struct IncreasePlayerLevelIxData;
impl IncreasePlayerLevelIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_PLAYER_LEVEL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INCREASE_PLAYER_LEVEL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_PLAYER_LEVEL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_player_level_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreasePlayerLevelKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_PLAYER_LEVEL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: IncreasePlayerLevelIxData.try_to_vec()?,
    })
}
pub fn increase_player_level_ix(
    keys: IncreasePlayerLevelKeys,
) -> std::io::Result<Instruction> {
    increase_player_level_ix_with_program_id(crate::ID, keys)
}
pub fn increase_player_level_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePlayerLevelAccounts<'_, '_>,
) -> ProgramResult {
    let keys: IncreasePlayerLevelKeys = accounts.into();
    let ix = increase_player_level_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_player_level_invoke(
    accounts: IncreasePlayerLevelAccounts<'_, '_>,
) -> ProgramResult {
    increase_player_level_invoke_with_program_id(crate::ID, accounts)
}
pub fn increase_player_level_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePlayerLevelAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreasePlayerLevelKeys = accounts.into();
    let ix = increase_player_level_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_player_level_invoke_signed(
    accounts: IncreasePlayerLevelAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_player_level_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn increase_player_level_verify_account_keys(
    accounts: IncreasePlayerLevelAccounts<'_, '_>,
    keys: IncreasePlayerLevelKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.player_account.key, keys.player_account),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_player_level_verify_writable_privileges<'me, 'info>(
    accounts: IncreasePlayerLevelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.player_account, accounts.signer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_player_level_verify_signer_privileges<'me, 'info>(
    accounts: IncreasePlayerLevelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_player_level_verify_account_privileges<'me, 'info>(
    accounts: IncreasePlayerLevelAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_player_level_verify_writable_privileges(accounts)?;
    increase_player_level_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EQUIP_PREMIUM_ITEM_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct EquipPremiumItemAccounts<'me, 'info> {
    pub player_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EquipPremiumItemKeys {
    pub player_account: Pubkey,
    pub admin: Pubkey,
}
impl From<EquipPremiumItemAccounts<'_, '_>> for EquipPremiumItemKeys {
    fn from(accounts: EquipPremiumItemAccounts) -> Self {
        Self {
            player_account: *accounts.player_account.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<EquipPremiumItemKeys> for [AccountMeta; EQUIP_PREMIUM_ITEM_IX_ACCOUNTS_LEN] {
    fn from(keys: EquipPremiumItemKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.player_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EQUIP_PREMIUM_ITEM_IX_ACCOUNTS_LEN]> for EquipPremiumItemKeys {
    fn from(pubkeys: [Pubkey; EQUIP_PREMIUM_ITEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            player_account: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<EquipPremiumItemAccounts<'_, 'info>>
for [AccountInfo<'info>; EQUIP_PREMIUM_ITEM_IX_ACCOUNTS_LEN] {
    fn from(accounts: EquipPremiumItemAccounts<'_, 'info>) -> Self {
        [accounts.player_account.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EQUIP_PREMIUM_ITEM_IX_ACCOUNTS_LEN]>
for EquipPremiumItemAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EQUIP_PREMIUM_ITEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            player_account: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const EQUIP_PREMIUM_ITEM_IX_DISCM: [u8; 8] = [11, 231, 6, 89, 240, 228, 242, 85];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EquipPremiumItemIxArgs {
    pub item_type: PremiumItemType,
    pub item_id: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EquipPremiumItemIxData(pub EquipPremiumItemIxArgs);
impl From<EquipPremiumItemIxArgs> for EquipPremiumItemIxData {
    fn from(args: EquipPremiumItemIxArgs) -> Self {
        Self(args)
    }
}
impl EquipPremiumItemIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EQUIP_PREMIUM_ITEM_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EQUIP_PREMIUM_ITEM_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EquipPremiumItemIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EQUIP_PREMIUM_ITEM_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn equip_premium_item_ix_with_program_id(
    program_id: Pubkey,
    keys: EquipPremiumItemKeys,
    args: EquipPremiumItemIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EQUIP_PREMIUM_ITEM_IX_ACCOUNTS_LEN] = keys.into();
    let data: EquipPremiumItemIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn equip_premium_item_ix(
    keys: EquipPremiumItemKeys,
    args: EquipPremiumItemIxArgs,
) -> std::io::Result<Instruction> {
    equip_premium_item_ix_with_program_id(crate::ID, keys, args)
}
pub fn equip_premium_item_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EquipPremiumItemAccounts<'_, '_>,
    args: EquipPremiumItemIxArgs,
) -> ProgramResult {
    let keys: EquipPremiumItemKeys = accounts.into();
    let ix = equip_premium_item_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn equip_premium_item_invoke(
    accounts: EquipPremiumItemAccounts<'_, '_>,
    args: EquipPremiumItemIxArgs,
) -> ProgramResult {
    equip_premium_item_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn equip_premium_item_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EquipPremiumItemAccounts<'_, '_>,
    args: EquipPremiumItemIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EquipPremiumItemKeys = accounts.into();
    let ix = equip_premium_item_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn equip_premium_item_invoke_signed(
    accounts: EquipPremiumItemAccounts<'_, '_>,
    args: EquipPremiumItemIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    equip_premium_item_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn equip_premium_item_verify_account_keys(
    accounts: EquipPremiumItemAccounts<'_, '_>,
    keys: EquipPremiumItemKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.player_account.key, keys.player_account),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn equip_premium_item_verify_writable_privileges<'me, 'info>(
    accounts: EquipPremiumItemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.player_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn equip_premium_item_verify_signer_privileges<'me, 'info>(
    accounts: EquipPremiumItemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn equip_premium_item_verify_account_privileges<'me, 'info>(
    accounts: EquipPremiumItemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    equip_premium_item_verify_writable_privileges(accounts)?;
    equip_premium_item_verify_signer_privileges(accounts)?;
    Ok(())
}