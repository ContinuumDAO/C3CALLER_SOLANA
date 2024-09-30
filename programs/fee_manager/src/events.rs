use anchor_lang::prelude::*;

#[event]
pub struct AddFeeTokenEvent {
    pub fee_token: Pubkey,
}

#[event]
pub struct DelFeeTokenEvent {
    pub fee_token: Pubkey,
}

#[event]
pub struct SetFeeTokenEvent {
    pub fee_token: Pubkey,
    pub src_chain_id: u64,
    pub dst_chain_id: u64,
    pub fee: u64,
    pub pay_from: u64,
}

#[event]
pub struct SetLiqFeeEvent {
    pub fee_token: Pubkey,
    pub fee: u64,
}

#[event]
pub struct WithdrawalEvent {
    pub fee_token: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
}