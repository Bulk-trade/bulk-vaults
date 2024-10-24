use crate::instruction::VaultInstruction;
use crate::instructions::{deposit, initialize_vault, withdraw};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VaultInstruction::unpack(instruction_data)?;
    match instruction {
        VaultInstruction::InitializeVault { vault_id } => {
            initialize_vault(program_id, accounts, vault_id)
        }
        VaultInstruction::Deposit {
            vault_id,
            user_pubkey,
            amount,
            fund_status,
            bot_status,
        } => deposit(
            program_id,
            accounts,
            vault_id,
            user_pubkey,
            amount,
            fund_status,
            bot_status,
        ),
        VaultInstruction::Withdraw {
            vault_id,
            user_pubkey,
            amount,
            fund_status,
            bot_status,
        } => withdraw(
            program_id,
            accounts,
            vault_id,
            user_pubkey,
            amount,
            fund_status,
            bot_status,
        ),
    }
}



