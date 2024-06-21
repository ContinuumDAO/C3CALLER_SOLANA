use crate::*;

#[event]
pub struct LogC3Call{
    pub dapp_id:u128,
    pub uuid: [u8;32],
    pub caller:Pubkey,
    pub to_chain_id:String,
    pub to:String,
    pub data:Vec<u8>,
    pub extra: Option<Vec<u8>>
}

#[event]
pub struct LogFallbackCall{
    dapp_id:u128,
    uuid: [u8;32],
    to:String,
    data:Vec<u8>,
    reasons: Vec<u8>
}

#[event]
pub struct LogExecCall{
    pub dapp_id:u128,
    pub to:String,
    pub uuid: [u8;32],
    pub from_chain_id:String,
    pub source_tx:String,
    pub data:Vec<u8>,
    pub success:bool,
    pub reason: Vec<u8>
}

#[event]
pub struct LogExecFallback{
    pub dapp_id:u128,
    pub to:String,
    pub uuid: [u8;32],
    pub from_chain_id:String,
    pub source_tx:String,
    pub data:Vec<u8>,
    pub reason: Vec<u8>
}





