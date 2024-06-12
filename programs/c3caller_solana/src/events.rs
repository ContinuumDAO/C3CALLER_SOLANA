use crate::*;

#[event]
pub struct LogC3Call{
    dapp_id:u128,
    uuid: [u8;32],
    caller:Pubkey,
    to_chain_id:String,
    to:String,
    data:Vec<u8>,
    extra: Vec<u8>
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
    dapp_id:u128,
    to:String,
    uuid: [u8;32],
    from_chain_id:String,
    source_tx:String,
    data:Vec<u8>,
    success:bool,
    reason: Vec<u8>
}

#[event]
pub struct LogExecFallback{
    dapp_id:u128,
    to:String,
    uuid: [u8;32],
    from_chain_id:String,
    source_tx:String,
    data:Vec<u8>,
    success:bool,
    reason: Vec<u8>
}





