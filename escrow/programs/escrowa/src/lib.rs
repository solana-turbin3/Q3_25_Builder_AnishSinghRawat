// declare_id!("33yRXuNJTyCv8B1PHnvL6Ht1kVZSi5LfTr7eFwmgcGGE");

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

declare_id!("33yRXuNJTyCv8B1PHnvL6Ht1kVZSi5LfTr7eFwmgcGGE");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
