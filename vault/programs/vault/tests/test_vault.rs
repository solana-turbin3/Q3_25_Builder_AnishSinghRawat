#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(unused)]

#[cfg(test)]
mod tests {
    use anchor_lang::prelude::*;
    use anchor_lang::InstructionData;
    use mollusk_svm::{program, result::Check, Mollusk};
    use solana_program::message;
    use solana_sdk::account::WritableAccount;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey,
    };
    use vault::instruction;
    use vault::VaultState;

    use mollusk_svm_bencher::MolluskComputeUnitBencher;

    //// disable logging (optional)
    // solana_logger::setup_with("");

    const ID: pubkey::Pubkey = pubkey!("Hot4S2HDwRYp4YVw1cEUqMcHkvRDWfPfAqLxGG7ndZbN");

    // Deterministic - GOOD for tests
    const USER: pubkey::Pubkey = pubkey::Pubkey::new_from_array([0x01; 32]);

    // Non-deterministic - BAD for tests
    // let USER = Pubkey::new_unique(); // Different every time!

    #[test]
    fn test_deposit() {
        //mollusk instance
        let mollusk = Mollusk::new(&ID, "../../target/deploy/vault");

        //Pubkeys of accounts needed
        let (state_pda, state_bump) =
            pubkey::Pubkey::find_program_address(&[b"state", USER.as_ref()], &ID);

        let (vault_pda, vault_bump) =
            pubkey::Pubkey::find_program_address(&[b"vault", state_pda.as_ref()], &ID);

        //system program acc
        let (system_program, system_account) = program::keyed_account_for_system_program();

        //create test accounts
        let user_account = Account::new(2 * LAMPORTS_PER_SOL, 0, &system_program);
        let state_account = Account::new(0, 0, &system_program);
        let vault_account = Account::new(0, 0, &system_program);

        ///BUILDING INSTRUCTION
        //define instruction data
        let init_data = instruction::Initialize {}.data();

        //define accounts for the instruction
        let init_ix_accs = vec![
            AccountMeta::new(USER, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        //create the instruction
        let init_ix = Instruction::new_with_bytes(ID, &init_data, init_ix_accs);

        let init_tx_accounts = &vec![
            //added & for mollusk_bench
            (USER, user_account.clone()),
            (state_pda, state_account),
            (vault_pda, vault_account),
            (system_program, system_account.clone()),
        ]; //user_account is used again later in the test (specifically in the deposit part of the test)
           //clone is used because of Rust ownership rules: Without clone(),
           //the ownership of user_account would be moved into the vector,
           //and you wouldn't be able to use it again

        // Execute initialize
        let result = mollusk.process_instruction(&init_ix, init_tx_accounts); //remmoved & before init_tx_accounts because added up top

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

        let initial_user_balance = 2 * LAMPORTS_PER_SOL;
        let initial_vault_balance = 0;

        let deposit_ix_accs = vec![
            AccountMeta::new(USER, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        //Build/create the Ix
        let instructions = Instruction::new_with_bytes(ID, &data, deposit_ix_accs);

        //Get Tx Accounts
        let tx_accs = &vec![
            (USER, post_user_account),
            (state_pda, post_state_account), //system_account.clone()
            (vault_pda, post_vault_account),
            (system_program, system_account),
        ];

        //process and validate our instruction
        // let test_deposit =
        mollusk.process_and_validate_instruction(&instructions, tx_accs, &[Check::success()]); //removed & from tx_accs

        MolluskComputeUnitBencher::new(mollusk)
            .bench(("Initialize", &init_ix, init_tx_accounts))
            .bench(("Deposit", &instructions, tx_accs))
            .must_pass(true)
            .out_dir("benches/")
            .execute();
    }

    #[test]
    fn test_withdraw() {
        let mollusk = Mollusk::new(&ID, "../../target/deploy/vault");

        let (state_pda, state_bump) =
            pubkey::Pubkey::find_program_address(&[b"state", USER.as_ref()], &ID);

        let (vault_pda, vault_bump) =
            pubkey::Pubkey::find_program_address(&[b"vault", state_pda.as_ref()], &ID);

        let (system_program, system_account) = program::keyed_account_for_system_program();

        // User account with some initial balance
        let user_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program); // 0.5 SOL
        let mut state_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(8 + VaultState::INIT_SPACE),
            8 + VaultState::INIT_SPACE,
            &ID,
        );
        let vault_account = Account::new(500_000_000, 0, &system_program);

        //inject data
        let initial_state = VaultState {
            vault_bump,
            state_bump,
        };

        let mut state_data = state_account.data_as_mut_slice();
        anchor_lang::AccountSerialize::try_serialize(&initial_state, &mut state_data);

        //get accs meta
        let withdraw_ix_accs = vec![
            AccountMeta::new(USER, true),
            AccountMeta::new(state_pda, true),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        //Data
        let withdraw_amount = 500_000_000;
        let data = (vault::instruction::Withdraw {
            amount: withdraw_amount,
        })
        .data();

        //build Ix
        let instructions = Instruction::new_with_bytes(ID, &data, withdraw_ix_accs);

        //Get Tx Accounts
        let tx_accs = &vec![
            (USER, user_account.clone()),
            (state_pda, state_account.clone()),
            (vault_pda, vault_account.clone()),
            (system_program, system_account.clone()),
        ];

        //process and validate our instruction
        let test_deposit =
            mollusk.process_and_validate_instruction(&instructions, &tx_accs, &[Check::success()]);

        // Inside your test_withdraw function, after setting up the withdraw instruction and accounts:
        MolluskComputeUnitBencher::new(mollusk)
            .bench(("Withdraw", &instructions, tx_accs))
            .must_pass(true)
            .out_dir("benches/") // Same directory is fine
            .execute();
    }

    #[test]
    fn test_close() {
        let mollusk = Mollusk::new(&ID, "../../target/deploy/vault");

        let (state_pda, state_bump) =
            pubkey::Pubkey::find_program_address(&[b"state", USER.as_ref()], &ID);

        let (vault_pda, vault_bump) =
            pubkey::Pubkey::find_program_address(&[b"vault", state_pda.as_ref()], &ID);

        let (system_program, system_account) = program::keyed_account_for_system_program();

        // Create initial accounts for initialization
        let user_account = Account::new(2 * LAMPORTS_PER_SOL, 0, &system_program);
        let state_account = Account::new(0, 0, &system_program);
        let vault_account = Account::new(0, 0, &system_program);

        // Step 1: Initialize the vault first
        let init_data = instruction::Initialize {}.data();
        let init_ix_accs = vec![
            AccountMeta::new(USER, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program, false),
        ];
        let init_ix = Instruction::new_with_bytes(ID, &init_data, init_ix_accs);

        let init_tx_accounts = vec![
            (USER, user_account),
            (state_pda, state_account),
            (vault_pda, vault_account),
            (system_program, system_account.clone()),
        ];

        // Execute initialize and get the result
        let init_result = mollusk.process_instruction(&init_ix, &init_tx_accounts);

        // Extract the initialized accounts from the result
        let post_user_account = init_result.get_account(&USER).unwrap().clone();
        let post_state_account = init_result.get_account(&state_pda).unwrap().clone();
        let mut post_vault_account = init_result.get_account(&vault_pda).unwrap().clone();

        // Add some funds to the vault for testing the close operation
        post_vault_account.set_lamports(500_000_000); // 0.5 SOL

        // Step 2: Create close instruction
        let close_data = instruction::Close {}.data();
        let close_ix_accs = vec![
            AccountMeta::new(USER, true),
            AccountMeta::new(state_pda, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program, false),
        ];
        let close_ix = Instruction::new_with_bytes(ID, &close_data, close_ix_accs);

        // Create accounts for close operation
        let close_tx_accounts = vec![
            (USER, post_user_account),
            (state_pda, post_state_account),
            (vault_pda, post_vault_account),
            (system_program, system_account),
        ];

        // Execute close instruction
        mollusk.process_and_validate_instruction(
            &close_ix,
            &close_tx_accounts,
            &[Check::success()],
        );

        // Optional: Add benchmarking
        MolluskComputeUnitBencher::new(mollusk)
            .bench(("Initialize", &init_ix, &init_tx_accounts))
            .bench(("Close", &close_ix, &close_tx_accounts))
            .must_pass(true)
            .out_dir("benches/")
            .execute();
    }
}
