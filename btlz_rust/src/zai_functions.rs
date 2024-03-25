use crate::AppState;
use solana_sdk::{
    pubkey::Pubkey,
    instruction::{Instruction, AccountMeta},
    system_program,
    transaction::Transaction,
    sysvar,
};
use std::sync::Arc;

pub async fn create_player(
    app_state: Arc<AppState>,
    signer_pubkey: Pubkey, // The public key of the user's wallet acting as the payer
    active_class: u64,
    active_weapon: u64,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let rpc_client = &app_state.solana_client;
    let program_id = app_state.program_id;

    // Assuming that your Anchor program expects a specific instruction format
    // Here, we construct the instruction data as a simple byte vector
    // The first byte could represent the instruction index if needed, followed by the actual data
    // This part is highly dependent on how your Anchor program expects to receive the instruction data
    let mut data = Vec::new();
    data.push(0); // For example, 0 could represent the `create_player` instruction index
    data.extend_from_slice(&active_class.to_le_bytes());
    data.extend_from_slice(&active_weapon.to_le_bytes());

    // Derive PDA for the player account
    let seeds = &[b"player", signer_pubkey.as_ref()];
    let (player_pda, bump_nonce) = Pubkey::find_program_address(seeds, &program_id);

    // Include the bump nonce in the data if required by your program
    data.push(bump_nonce);

    // Define the accounts involved in the transaction
    let accounts = vec![
        AccountMeta::new(player_pda, false), // The player PDA
        AccountMeta::new_readonly(signer_pubkey, true), // Marking the signer
        AccountMeta::new_readonly(sysvar::rent::id(), false), // Rent sysvar for account creation
        AccountMeta::new_readonly(system_program::ID, false), // System program for account creation
    ];

    let instruction = Instruction {
        program_id,
        accounts,
        data,
    };

    // Create the transaction
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&signer_pubkey)
    );

    Ok(transaction)
}
