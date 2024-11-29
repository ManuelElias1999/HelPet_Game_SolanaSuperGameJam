use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program::{invoke},
    program_error::ProgramError,
};
use spl_token::{instruction as token_instruction};

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
        0 => unlock_animal(accounts, instruction_data), // Unlock animal
        1 => spend_tokens(accounts, instruction_data), // Spend tokens
        _ => {
            msg!("Instruction not recognized.");
            Err(ProgramError::InvalidInstructionData)
        }
    }
}

/// Unlock a new animal by spending HPET tokens
/// @param accounts - Array containing:
///   - Player's account
///   - Player's HPET token account
///   - SPL Token program
///   - Animal state account (tracks unlocked animals)
/// @param instruction_data - The instruction data containing operation details
fn unlock_animal(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let player_account = next_account_info(account_info_iter)?; // Player's account
    let hpet_token_account = next_account_info(account_info_iter)?; // Player's HPET token account
    let token_program = next_account_info(account_info_iter)?; // SPL Token Program
    let animal_state_account = next_account_info(account_info_iter)?; // Animal unlock state account

    let required_hpet = 100; // Required tokens to unlock an animal

    // Verify player has enough tokens before transfer
    let token_balance = TokenAccount::unpack(&hpet_token_account.try_borrow_data()?)?;
    if token_balance.amount < required_hpet {
        return Err(ProgramError::InsufficientFunds);
    }

    // Transfer HPET from player to contract to unlock the animal
    invoke(
        &token_instruction::transfer(
            token_program.key,
            hpet_token_account.key,
            animal_state_account.key,
            &[],
            required_hpet,
        )?,
        &[
            hpet_token_account.clone(),
            animal_state_account.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Animal successfully unlocked for player.");
    Ok(())
}

/// Spend tokens on additional features or functionalities
/// @param accounts - Array containing:
///   - Player's account
///   - Player's HPET token account  
///   - SPL Token program
/// @param instruction_data - The instruction data containing:
///   - [0] Operation code
///   - [1] Amount of tokens to spend
fn spend_tokens(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let player_account = next_account_info(account_info_iter)?; // Player's account
    let hpet_token_account = next_account_info(account_info_iter)?; // Player's HPET token account
    let token_program = next_account_info(account_info_iter)?; // SPL Token Program

    let amount = instruction_data[1] as u64; // Amount to spend (extracted from instruction_data)

    // Verify player has enough tokens
    let token_balance = TokenAccount::unpack(&hpet_token_account.try_borrow_data()?)?;
    if token_balance.amount < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    // Transfer spent tokens to corresponding contract
    invoke(
        &token_instruction::transfer(
            token_program.key,
            hpet_token_account.key,
            player_account.key,
            &[],
            amount,
        )?,
        &[
            hpet_token_account.clone(),
            player_account.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Tokens spent successfully.");
    Ok(())
}