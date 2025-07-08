#![allow(unexpected_cfgs)]
#![allow(unused_imports)]
#![allow(deprecated)]
use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;


declare_id!("Hot4S2HDwRYp4YVw1cEUqMcHkvRDWfPfAqLxGG7ndZbN");            // Declares at what onchain address the program lives.

#[program]                          //Specifies the module containing the program’s instruction logic
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}


#[account]    // Applied to structs to create custom account types for the program
// #[derive(InitSpace, Debug)] // needed to automatically determine the size of the account
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 1 + 1;  
}

#[derive(Accounts)]                 //Lists the accounts an instruction requires and enforces their constraints automatically
pub struct Initialize<'info> {
    #[account(mut)] // this marked as mutable because the balance of signer would be decreased
    pub signer: Signer<'info>, // represent the account that would sign the transaction

    // we would almost always need the signer
    #[account(
        init, 
        payer = signer, 
        space = 8 + VaultState::INIT_SPACE ,
        seeds = [b"state", signer.key().as_ref()], // &str -> bytes
        bump,
    )]

    // the Account field also ensures that the account provided is owned by the program
    pub vault_state: Account<'info, VaultState>,    //Account container that checks ownership on deserialization


    #[account(
        mut,
        seeds = [b"vault" ,vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>, // represent any account owned by the system program

    pub system_program: Program<'info, System>, // needed because an account creation happens and this is done by the system program
}

impl<'info> Initialize<'info> {
    // anchor creates a <AccountStructName>Bumps struct that holds the bump values for the seeds
    pub fn  initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        
        let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, rent_exempt)?;

        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;

        // following the set_inner patterns feels cleaner and more idiomatic
        // set_inner is defined in the impl block of the Account type

        msg!("ℹ About to Initialize the Vault State Account");

        // self.vault_state.set_inner( VaultState {
        //     vault_bump: bumps.vault,
        //     state_bump: bumps.vault_state,
        // });
        
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    // can't close this account with close constraint as it is a SystemAccount and not PDA

    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
    
impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // we need the account info of the system program to use in a CPI call
        let system_program = self.system_program.to_account_info();

        // the accounts needed for the CPI, the Transfer struct is an Account struct for account validation
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };
        
        // CPI Context basically takes in the program we wish to call and an account struct that holds the accounts needed for the CPI
        let cpi_ctx = CpiContext::new(system_program, accounts);

        // then we can call the instruction with the cpi_ctx and any additional parameters
        transfer(cpi_ctx,  amount)?;

        Ok(())
    }

    pub fn withdraw(&mut self, _amount: u64) -> Result<()> {
        let system_program = self.system_program.to_account_info();

        let accounts = Transfer {
            from: self.vault.to_account_info(), // here the vault is the from account, so we must sign with the signer account
            to: self.signer.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(system_program, accounts, signer_seeds);
        
        transfer(cpi_ctx, _amount)?;

        Ok(())
    }
} 


#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        close = signer, // close the account and send the lamports to the signer
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}


impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let system_program = self.system_program.to_account_info();

        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(system_program, accounts, signer_seeds);

        transfer(cpi_ctx, self.vault.lamports())?;

        Ok(())
    }
}