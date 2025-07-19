#![allow(unexpected_cfgs)]
#![allow(unused_imports)]
#![allow(deprecated)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6yox4XuCFsj75z2zNBtmY6GAiTz1z8ttpzpWyy6sgPzo");

#[program]
pub mod escrow {
    use super::*;

    // pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
    //     ctx.accounts.deposit()
    // }
}

#[derive(Accounts)]
pub struct Initialize {

}
