use anchor_lang::prelude::*;

declare_id!("3TUgwu7jzPcB2AnLPRtT7f4pqar31NKd472nNhRyYcQR");

#[program]
pub mod hel_pet_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
