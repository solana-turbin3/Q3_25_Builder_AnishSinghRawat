use crate::state::Bet;
use anchor_lang::{
    prelude::*, solana_program, system_program::{transfer, Transfer}
};
use solana_program::{
    ed25519_program,
    hash::hash,
    sysvar::instructions::load_instruction_at_checked,
};

use crate::{errors::DiceError, state::Bet}; // Note: 'DiceError' is currently unused

pub const HOUSE_EDGE: u16 = 150; // 1.5% House edge

#[derive(Accounts)]
pub struct Resolvebet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    //
    pub player: UncheckedAccount<'info>,

    #[account(
        mut,
        close = player,
        has_one = player,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,
    
    #[account(
        mut,        //no need for init cuz system account
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}