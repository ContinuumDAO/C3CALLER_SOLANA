use crate::*;
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