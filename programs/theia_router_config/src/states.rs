use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TokenInfo {
    pub addr: Pubkey,
    pub decimals: u8,
    pub to_chain_addr: Pubkey,
    pub to_chain_decimals: u8,
}

#[account]
pub struct TokenIdList {
    pub token_ids: Vec<String>,
}

#[account]
#[derive(InitSpace)]
pub struct TokenIDMap {
    pub is_valid: bool,
}
#[account]
pub struct ChainConfig {
    pub chain_id: u64,
    pub blockchain: String,
    pub router_contract: String,
    pub extra: String,
}
#[account]
#[derive(InitSpace)]
pub struct TokenConfig {
    pub chain_id: u64,
    pub decimals: u8,
    #[max_len(50)]
    pub contract_address: String,
    pub contract_version: u64,
    #[max_len(50)]
    pub router_contract: String,
    #[max_len(50)]
    pub extra: String,
}

#[account]
pub struct SwapConfig {
    pub from_chain_id: u64,
    pub to_chain_id: u64,
    pub maximum_swap: u64,
    pub minimum_swap: u64,
}


#[account]
pub struct MultichainToken {
    pub chain_id: u64,
    pub token_address: String,
}