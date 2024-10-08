use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::{IsInitialized, Sealed};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VaultAccountState {
    pub is_initialized: bool,
    pub user_pubkey: String,
    pub amount: u32,
    pub fund_status: String,
    pub bot_status: String,
}

impl Sealed for VaultAccountState {}

impl IsInitialized for VaultAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
