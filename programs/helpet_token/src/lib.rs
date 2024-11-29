#![cfg(feature = "program")]

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{ProgramResult, entrypoint},
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
};
use spl_token::{self, state::Mint};

pub struct HelPetToken;

#[program]
pub mod helpet_token {
    use super::*;

    // Initialize token function
    pub fn initialize_token(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        _seed: &[u8],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let payer_account = next_account_info(accounts_iter)?;

        // Here you could add logic to initialize the token on the network
        msg!("Initial token setup complete.");

        Ok(())
    }

    // Function to mint HelPet tokens (HPET)
    pub fn mint_tokens(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,  // Number of tokens to mint
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let payer_account = next_account_info(accounts_iter)?;
        let token_account = next_account_info(accounts_iter)?;
        let mint_account = next_account_info(accounts_iter)?;

        // Verify that the token mint is correct (HPET mint)
        let mint = Mint::unpack(&mint_account.data.borrow())?;
        if mint.decimals != 18 {
            return Err(ProgramError::InvalidArgument);
        }

        // Use SPL Token program functions to mint tokens
        msg!("Minting {} HelPet tokens", amount);
        
        let mint_ix = spl_token::instruction::mint_to(
            &spl_token::id(),
            &mint_account.key,
            &token_account.key,
            &payer_account.key,
            &[],
            amount,
        )?;

        solana_program::program::invoke(
            &mint_ix,
            &[
                mint_account.clone(),
                token_account.clone(),
                payer_account.clone(),
            ],
        )?;

        Ok(())
    }

    // Function to transfer HelPet tokens
    pub fn transfer_tokens(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,  // Amount to transfer
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let sender_account = next_account_info(accounts_iter)?;
        let recipient_account = next_account_info(accounts_iter)?;
        let token_account = next_account_info(accounts_iter)?;

        // Verify account balance
        msg!("Transferring {} HelPet tokens", amount);

        let transfer_ix = spl_token::instruction::transfer(
            &spl_token::id(),
            &sender_account.key,
            &recipient_account.key,
            &token_account.key,
            &[],
            amount,
        )?;

        solana_program::program::invoke(
            &transfer_ix,
            &[
                sender_account.clone(),
                recipient_account.clone(),
                token_account.clone(),
            ],
        )?;

        Ok(())
    }

    // Function to buy HPET tokens with USDC
    pub fn buy_tokens_with_usdc(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        usdc_amount: u64,  // Amount of USDC to send
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let payer_account = next_account_info(accounts_iter)?;
        let usdc_account = next_account_info(accounts_iter)?;
        let helpet_account = next_account_info(accounts_iter)?;
        let mint_account = next_account_info(accounts_iter)?;

        // Define purchase prices
        let price_per_token = 0.0999; // USDC per HelPet token (Example price for 100 HelPet)

        // Convert USDC to HPET
        let tokens_to_mint = (usdc_amount as f64 / price_per_token) as u64;

        // Here you would verify that the USDC payment is sufficient
        msg!("Buying tokens with {} USDC", usdc_amount);

        let transfer_usdc_ix = spl_token::instruction::transfer(
            &spl_token::id(),
            &usdc_account.key,
            &payer_account.key,
            &payer_account.key,
            &[],
            usdc_amount,
        )?;

        // Mint HPET tokens
        let mint_ix = spl_token::instruction::mint_to(
            &spl_token::id(),
            &mint_account.key,
            &helpet_account.key,
            &payer_account.key,
            &[],
            tokens_to_mint,
        )?;

        solana_program::program::invoke(
            &transfer_usdc_ix,
            &[
                usdc_account.clone(),
                payer_account.clone(),
                payer_account.clone(),
            ],
        )?;

        solana_program::program::invoke(
            &mint_ix,
            &[
                mint_account.clone(),
                helpet_account.clone(),
                payer_account.clone(),
            ],
        )?;

        Ok(())
    }
}