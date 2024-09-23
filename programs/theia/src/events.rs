use crate::*;

#[event]
pub struct LogChangeUUIDKeeper {
    pub uuid_keeper: Pubkey,
}

#[event]
pub struct LogChangeTheiaConfig {
    pub theia_config: Pubkey,
}

#[event]
pub struct LogChangeFeeManager {
    pub fee_manager: Pubkey,
}

#[event]
pub struct LogTheiaCross{
    pub token: String,
    pub from: String,
    pub swapout_id: [u8; 32],
    pub amount: u64,
    pub from_chain_id: u64,
    pub to_chain_id: u64,
    pub fee: u64,
    pub fee_token: String,
    pub receiver: String,

}


