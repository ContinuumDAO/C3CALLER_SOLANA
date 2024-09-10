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