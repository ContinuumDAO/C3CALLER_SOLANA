use std::io::Bytes;

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