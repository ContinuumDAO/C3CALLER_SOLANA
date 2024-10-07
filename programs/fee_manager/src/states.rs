use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FromFeeConfig{
    fee:u64
}

#[account]
#[derive(InitSpace)]
pub struct ToFeeConfig{
    fee:u64
}