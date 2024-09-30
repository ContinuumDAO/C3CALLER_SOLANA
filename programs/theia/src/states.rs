use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CrossAuto {
    pub target: String,
    pub receiver: String,
    pub amount: u64,
    pub fee_amount: u64,
    pub to_chain_id: u64,
    pub token_id: String,
    pub fee_token_id: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CrossNonEvm {
    pub amount: u64,
    pub fee_amount: u64,
    pub to_chain_id: u64,
    pub target: String,
    pub receiver: String,
    pub token_id: String,
    pub fee_token_id: String,
    pub call_data: Vec<u8>,
    pub extra: Vec<u8>,
    pub is_eth_sign: bool,
    pub signature: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct TokenInfo {
    pub addr: String,
    pub decimals: u8,
    pub to_chain_addr: String,
    pub to_chain_decimals: u8,
}