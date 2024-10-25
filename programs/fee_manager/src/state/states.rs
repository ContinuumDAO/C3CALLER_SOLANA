use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FromFeeConfig{
   pub  fee:u64
}

#[account]
#[derive(InitSpace)]
pub struct ToFeeConfig{
   pub  fee:u64
}