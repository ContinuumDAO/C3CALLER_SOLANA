use anchor_lang::prelude::*;
use std::mem::size_of;
mod errors;
mod events;
mod states;
mod utils;
use crate::errors::C3CallerErros;
use crate::events::*;
use crate::states::*;
declare_id!("EQPiEmWVmpkD53LfBahxY8gFaEiDzsjobLKuyxDmPHy9");

pub const C3UUID_KEEPER_SEED: &[u8] = b"c3uuidseed";
pub const PAUSE_SEED: &[u8] = b"pauseseed";
pub const SET_PAUSE_SEED:&[u8] = b"setpauseseed";

#[program]
pub mod c_3caller_solana {
    use states::C3EvmMessage;
    use utils::gen_uuid;

    use super::*;

    pub fn initialize(ctx: Context<InitC3Caller>) -> Result<()> {
        ctx.accounts.pause.is_paused = false;
        Ok(())
    }

    pub fn c3call(ctx: Context<C3CallerState>,_dapp_id:u128,_caller:Pubkey,_to:String, _to_chain_id:String,_data:Vec<u8>,_extra:Vec<u8>)-> Result<()> {

        require!(_dapp_id>0,C3CallerErros::DappIdisEmpty);
        require!(_to.len()>0, C3CallerErros::ToisEmpty);
        require!(_to_chain_id.len()>0,C3CallerErros::ToChainIdisEmpty);
        require!(_data.len()>0,C3CallerErros::DataisEmpty);
        require!(!ctx.accounts.pause.is_paused,C3CallerErros::ContractIsPaused);


        let nonce = ctx.accounts.c3_uuid.current_nonce+1;
        let _uuid = gen_uuid(ctx.accounts.signer.key(), *ctx.program_id, &nonce, &_dapp_id, &_to, &_to_chain_id, &_data) ;

        emit!(LogC3Call{
            dapp_id:_dapp_id,
            uuid:_uuid,
            caller:_caller,
            to_chain_id:_to_chain_id,
            to:_to,
            data:_data,
            extra:Some(_extra),
        });

        Ok(())
        
    }

    pub fn c3broadcast(ctx: Context<C3CallerState>,_dapp_id:u128,_caller:Pubkey,_to:Vec<String>, _to_chain_ids:Vec<String>,_data:Vec<u8>)->Result<()>{
        require!(_dapp_id>0,C3CallerErros::DappIdisEmpty);
        require!(_to.len()>0, C3CallerErros::ToisEmpty);
        require!(&_to_chain_ids.len()>&0,C3CallerErros::ToChainIdisEmpty);
        require!(&_data.len()>&0,C3CallerErros::DataisEmpty);
       // require!(&_data.len() == &_to_chain_ids.len(),C3CallerErros::CallDataLengthMismatch);
        require!(!ctx.accounts.pause.is_paused,C3CallerErros::ContractIsPaused);

        let mut nonce = ctx.accounts.c3_uuid.current_nonce;
            for i in 0 .. _to_chain_ids.len()  {
                 nonce +=1;
                let _uuid = gen_uuid(ctx.accounts.signer.key(), *ctx.program_id, &nonce, &_dapp_id, &_to[i], &_to_chain_ids[i], &_data) ;

                emit!(LogC3Call{
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

    pub fn execute(ctx: Context<InitC3Caller>, _dapp_id:u128, _tx_sender:String,_message:C3EvmMessage)-> Result<()>{

        require!(_message.data.len() >0,C3CallerErros::DataisEmpty);
        require!(!ctx.accounts.pause.is_paused,C3CallerErros::ContractIsPaused);
       
       //TODO FINISH IMPLEMENTATION
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
    pub fn c3_fallback(ctx: Context<InitC3Caller>, _dapp_id:u128, _tx_sender:Pubkey, _message:C3EvmMessage)-> Result<()>{
    
        require!(_message.data.len()>0,C3CallerErros::DataisEmpty);
        require!(!ctx.accounts.pause.is_paused,C3CallerErros::ContractIsPaused);
            //TODO FINISH IMPLEMENTATION
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

    pub fn set_pause_state(ctx: Context<SetPause>,_pause:bool)->Result<()>{
            msg!("Current state {}",ctx.accounts.pause.is_paused);
            ctx.accounts.pause.is_paused = _pause;
            msg!("contract paused?: {}",_pause);
            Ok(())
    }


}
#[event_cpi]
#[derive(Accounts)]
pub struct InitC3Caller<'info> {
    #[account(init,
    payer = signer,
    space= size_of::<C3UUIDKeeper>()+8,
    seeds =[C3UUID_KEEPER_SEED],
    bump)]
    pub c3_uuid : Account<'info, C3UUIDKeeper>,
    #[account(init,
         payer = signer,
        space = 8+1,
        seeds = [PAUSE_SEED],
        bump
        )]
    pub pause:Account<'info,Pause>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program:Program<'info,System>,

}

#[event_cpi]
#[derive(Accounts)]
pub struct C3CallerState<'info>{
    #[account()]
    pub pause:Account<'info,Pause>,
    #[account(mut)]
    pub c3_uuid : Account<'info, C3UUIDKeeper>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetPause<'info>{
    #[account(mut,seeds=[PAUSE_SEED], bump)]
    pub pause:Account<'info,Pause>

}

