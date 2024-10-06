use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TokenInfo {
    pub addr: Pubkey,
    pub decimals: u8,
    pub to_chain_addr: Pubkey,
    pub to_chain_decimals: u8,
}