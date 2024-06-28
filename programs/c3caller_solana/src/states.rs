use std::{collections::HashMap, io::Bytes};
use anchor_lang::{accounts::account, solana_program::{keccak::hash, pubkey::Pubkey}};
use crate::*;

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct C3EvmMessage {
    pub uuid:[u8;32],
    pub to:String,
    pub from_chain_id:String,
    pub source_tx:String,
    pub fallback_to:String,
    pub data:Vec<u8>,
}
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct  C3Context{
    pub swap_id:[u8;32],
    pub from_chain_id:String,
    pub source_tx:String
}


#[account]
pub struct  Pause{
    pub is_paused:bool
}
#[account]

pub struct C3UUIDKeeper{
     pub current_nonce: u64,
     uuid_2_nonce: [u8; 32],
}

impl C3UUIDKeeper {
    pub fn gen_uuid(sender:Pubkey,program_id:Pubkey,nonce: u64,dapp_id:u128,to:String, to_chain_id:String, data:[u8;32])-> [u8;32]{
        hash(
            &[
                &sender.to_bytes()[..],
                &program_id.to_bytes()[..],
                &nonce.to_be_bytes()[..],
                &dapp_id.to_be_bytes()[..],
                &to.as_bytes()[..],
                &to_chain_id.as_bytes()[..],
                &data[..]
            ]
            .concat()
        )
        .to_bytes()
    
}
}