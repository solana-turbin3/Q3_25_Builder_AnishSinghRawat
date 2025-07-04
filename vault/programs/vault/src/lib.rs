use anchor_lang::prelude::*;

declare_id!("G9TUjEYHZsAunSKWW5FQSb5UDc8b7Tx9fnP81f77XvvP");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
