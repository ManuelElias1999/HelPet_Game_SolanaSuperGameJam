use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program::{invoke, invoke_signed},
    program_pack::{Pack, IsInitialized},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::{instruction as token_instruction, state::Account as TokenAccount};

/// Program's entrypoint
entrypoint!(process_instruction);

/// Process instructions sent to the contract
/// @param program_id - The program ID
/// @param accounts - The accounts required for the instruction
/// @param instruction_data - The instruction data, where first byte is the operation code
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data[0]; // First position: operation code
    match instruction {
        0 => collect_bones(program_id, accounts), // Collect bones
        1 => unlock_animal(program_id, accounts), // Unlock animals
        _ => {
            msg!("Instruction not recognized.");
            return Err(solana_program::program_error::ProgramError::InvalidInstructionData);
        }
    }
}

/// Collect bones and reward HPET tokens
/// This function mints HPET tokens to the player's account when they collect bones
/// @param program_id - The program ID
/// @param accounts - Array containing:
///   - Player's account
///   - HPET mint account
///   - Player's token account
///   - SPL Token program
fn collect_bones(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account = next_account_info(account_info_iter)?; // Player's account
    let hpet_mint = next_account_info(account_info_iter)?; // HelPet Token mint
    let hpet_token_account = next_account_info(account_info_iter)?; // Player's token account
    let token_program = next_account_info(account_info_iter)?; // SPL Token Program
    
    let amount = 10; // For example, 10 HPET tokens per bone collected

    // Call mint instruction
    invoke(
        &token_instruction::mint_to(
            token_program.key,
            hpet_mint.key,
            hpet_token_account.key,
            payer_account.key,
            &[],
            amount,
        )?,
        &[
            payer_account.clone(),
            hpet_mint.clone(),
            hpet_token_account.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Minted {} HPET tokens to player.", amount);
    Ok(())
}

/// Unlock animals by spending HPET tokens
/// This function transfers HPET tokens from player's account as payment for unlocking an animal
/// @param program_id - The program ID
/// @param accounts - Array containing:
///   - Player's account
///   - Player's token account
///   - SPL Token program
fn unlock_animal(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let player_account = next_account_info(account_info_iter)?; // Player's account
    let hpet_token_account = next_account_info(account_info_iter)?; // Player's token account
    let token_program = next_account_info(account_info_iter)?; // SPL Token Program
    
    let cost = 100; // For example, 100 tokens to unlock an animal

    // Call transfer instruction
    invoke(
        &token_instruction::transfer(
            token_program.key,
            hpet_token_account.key,
            player_account.key,
            &[],
            cost,
        )?,
        &[
            hpet_token_account.clone(),
            player_account.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Animal unlocked for player.");
    Ok(())
}