use crate::*;


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct EvmData{
    pub token:String,
    pub from:String,
    pub amount:u64,
    pub receiver:String,
    pub to_chain_id:String,
    
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct NonEvmData{
    pub token:String,
    pub from:String,
    pub amount:u64,
    pub receiver:String,
    pub to_chain_id:String,
    pub call_data: [u8;32],
}