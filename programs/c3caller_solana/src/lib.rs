use anchor_lang::prelude::*;
mod errors;
mod events;
mod states;
use crate::errors::C3CallerErros;
use crate::events::*;
use crate::states::*;
declare_id!("2hFPiv6jk5sEF6gai3v1q76oUQXPnghVNJF3GqwdN1hh");

#[program]
pub mod c3caller_solana {
    use states::C3EvmMessage;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        require!(ctx.remaining_accounts.len()>0,C3CallerErros::DappIdisEmpty);
        Ok(())
    }

    pub fn c3call(ctx: Context<Initialize>,_dapp_id:u128,_caller:Pubkey,_to:String, _to_chain_id:String,_data:Vec<u8>,_extra:Vec<u8>)-> Result<()> {

        require!(_dapp_id>0,C3CallerErros::DappIdisEmpty);
        require!(_to.len()>0, C3CallerErros::ToisEmpty);
        require!(_to_chain_id.len()>0,C3CallerErros::ToChainIdisEmpty);
        require!(_data.len()>0,C3CallerErros::DataisEmpty);

        let _uuid: [u8;32] =[0;32] ;

        emit_cpi!(LogC3Call{
            dapp_id:_dapp_id,
            uuid:_uuid,
            caller:_caller,
            to_chain_id:_to_chain_id,
            to:_to,
            data:_data.clone(),
            extra:Some(_extra),
        });


        Ok(())
        
    }

    pub fn c3broadcast(ctx: Context<Initialize>,_dapp_id:u128,_caller:Pubkey,_to:Vec<String>, _to_chain_ids:Vec<String>,_data:Vec<u8>)->Result<()>{
        require!(_dapp_id>0,C3CallerErros::DappIdisEmpty);
        require!(_to.len()>0, C3CallerErros::ToisEmpty);
        require!(_to_chain_ids.len()>0,C3CallerErros::ToChainIdisEmpty);
        require!(_data.len()>0,C3CallerErros::DataisEmpty);
        require!(_data.len() == _to_chain_ids.len(),C3CallerErros::CallDataLengthMismatch);


            for i in 0 .. _to_chain_ids.len()  {
                let _uuid: [u8;32] =[0;32] ;

                emit_cpi!(LogC3Call{
                    dapp_id:_dapp_id,
                    uuid:_uuid,
                    caller:_caller,
                    to_chain_id:_to_chain_ids[i].clone(),
                    to:_to[i].clone(),
                    data:_data.clone(),
                    extra:None,
        
                });
            }

        Ok(())
    }

    pub fn execute(ctx: Context<Initialize>, _dapp_id:u128, _tx_sender:String,_message:C3EvmMessage)-> Result<()>{

        require!(_message.data.len() >0,C3CallerErros::DataisEmpty);
       

        emit_cpi!(LogExecCall{
            dapp_id:_dapp_id,
            to:_message.to,
            uuid:_message.uuid,
            from_chain_id:_message.from_chain_id,
            source_tx:_message.source_tx,
            data:_message.data,
            success:true,
            reason:Vec::new()

        });
        Ok(())
    }
    pub fn c3Fallback(ctx: Context<Initialize>, _dapp_id:u128, _tx_sender:Pubkey, _message:C3EvmMessage)-> Result<()>{
    
        require!(_message.data.len()>0,C3CallerErros::DataisEmpty);

       // emit_cpi!(LogExecFallback{})
       let context: C3Context = C3Context{
        swap_id: _message.uuid,
        from_chain_id: _message.from_chain_id.clone(),
        source_tx: _message.source_tx.clone(),
    };

    emit_cpi!(LogExecFallback{
        dapp_id:_dapp_id,
        to:_message.to,
        uuid:_message.uuid,
        from_chain_id:_message.from_chain_id,
        source_tx:_message.source_tx,
        data:_message.data,
        reason: Vec::new()


    });


       Ok(())

    }


}
#[event_cpi]
#[derive(Accounts)]
pub struct Initialize {}
