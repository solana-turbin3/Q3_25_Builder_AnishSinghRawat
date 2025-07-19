#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(unused)]

#[cfg(test)]
mod tests {
    use anchor_lang::prelude::*; 
    use anchor_lang::InstructionData;
    use vault::instruction;
    use mollusk_svm::{program, Mollusk, result::Check};
    use solana_sdk::{
        account::Account, 
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL, 
        pubkey,
    };

    // use {
    //     mollusk_svm_bencher::MolluskComputeUnitBencher,
    // };
    //// disable logging (optional)
    // solana_logger::setup_with("");

    const ID: pubkey::Pubkey = pubkey!("Hot4S2HDwRYp4YVw1cEUqMcHkvRDWfPfAqLxGG7ndZbN");
    const USER: pubkey::Pubkey = pubkey::Pubkey::new_from_array([0x01; 32]);
    
    #[test]
    fn test_deposit() {
        //mollusk instance
        let mollusk = Mollusk::new(&ID, "../../target/deploy/vault");

        //Pubkeys
        let (state_pda, state_bump) = 
            pubkey::Pubkey::find_program_address(&[b"state", USER.as_ref()], &ID);

        let (vault_pda, vault_bump) = 
            pubkey::Pubkey::find_program_address(&[b"vault", state_pda.as_ref()], &ID);

        let (system_program, system_account) = program::keyed_account_for_system_program();

        //Build the accounts
        let user_account = Account::new(2 * LAMPORTS_PER_SOL, 0, &system_program);
        let state_account = Account::new(0, 0, &system_program);
        let vault_account = Account::new(0, 0, &system_program);

        //get the accounts meta
        let init_ix_accs = vec![
            AccountMeta::new(USER, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        let init_data = instruction::Initialize {}.data();
        let init_ix = Instruction::new_with_bytes(ID, &init_data, init_ix_accs);

        let init_tx_accounts = vec![
            (USER, user_account.clone()),
            (state_pda, state_account),
            (vault_pda, vault_account),
            (system_program, system_account.clone()),
        ];

        // Execute initialize
        let result = mollusk.process_instruction(&init_ix, &init_tx_accounts);

        // Step 2: Extract accounts after initialize for deposit test
        // Extract updated accounts from the result
        let post_user_account = result.get_account(&USER).unwrap().clone();
        let post_state_account = result.get_account(&state_pda).unwrap().clone();
        let post_vault_account = result.get_account(&vault_pda).unwrap().clone();

        //Data
        let deposit_amount: u64 = 500_000_000;
        let data = (vault::instruction::Deposit {
            amount: deposit_amount,
        })
        .data();

        let deposit_ix_accs = vec![
            AccountMeta::new(USER, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        //Build Ix
        let instructions = Instruction::new_with_bytes(ID, &data, deposit_ix_accs);

        //Get Tx Accounts
        let tx_accs = &vec![
            (USER, post_user_account),
            (state_pda, post_state_account),    //system_account.clone()
            (vault_pda, post_vault_account),
            (system_program, system_account),
        ];

        //process and validate our instruction
        // let test_deposit = 
        mollusk.process_and_validate_instruction(&instructions, &tx_accs, &[Check::success()]);
    }

    #[test]
    fn test_withdraw() {
        // Example mollusk test - uncomment imports when you need them
        // use mollusk_svm::{result::Check, Mollusk};
        // use anchor_lang::{InstructionData, ToAccountMetas};
        // use solana_program::instruction::Instruction;
        
        // let program_id = vault::id();
        // println!("Program ID: {}", program_id);
        
        // Add your mollusk test logic here
    }
}