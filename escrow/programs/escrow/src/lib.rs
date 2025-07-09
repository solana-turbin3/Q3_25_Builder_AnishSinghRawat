#![allow(unexpected_cfgs)]
#![allow(unused_imports)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

declare_id!("529nD1ZkbHg5fWJT4gcBNZJtpms79szRRFNY9E4Di8s6");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
