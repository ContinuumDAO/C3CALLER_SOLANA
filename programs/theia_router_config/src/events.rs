use anchor_lang::prelude::*;

#[event]
pub struct LogSetChainConfig {
    pub chain_id: u64,
    pub blockchain: String,
    pub router_contract: String,
    pub extra: String,
}
#[event]
pub struct LogSetTokenConfig {
    pub chain_id: u64,
    pub decimals: u8,
    pub contract_version: u64,
    pub token_id: String,
    pub contract_address: String,
    pub router_contract: String,
    pub extra: String,
}