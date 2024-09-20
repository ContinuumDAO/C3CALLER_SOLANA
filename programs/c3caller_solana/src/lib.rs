use anchor_lang::prelude::*;
use std::mem::size_of;
mod errors;
mod events;
pub mod states;
mod utils;
use crate::errors::C3CallerErros;
use crate::events::*;
use crate::states::*;
declare_id!("GYtoknPePL58qaXtaRmp4annpnAvPrYcdvv9ghtbLKoL");

pub const C3UUID_KEEPER_SEED: &[u8] = b"c3uuidseed";
pub const PAUSE_SEED: &[u8] = b"pauseseed";
pub const OWNER_KEY_SEED:&[u8] = b"ownerkeyseed";

#[program]
pub mod c_3caller_solana {
    use anchor_lang::solana_program::{instruction::Instruction, program::invoke};
    use states::C3SolMessage;
    use utils::gen_uuid;

    use super::*;

    pub fn initialize(ctx: Context<InitC3Caller>) -> Result<()> {
        ctx.accounts.pause.is_paused = false;
        ctx.accounts.owner.onwer_key = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn ccall(ctx: Context<C3CallerState>,_dapp_id:u128,_caller:Pubkey,_to:String, _to_chain_id:String,_data:Vec<u8>,_extra:Vec<u8>)-> Result<()> {

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

    pub fn cbroadcast(ctx: Context<C3CallerState>,_dapp_id:u128,_caller:Pubkey,_to:Vec<String>, _to_chain_ids:Vec<String>,_data:Vec<u8>)->Result<()>{
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

    #[access_control(check_owner(&ctx))]
    pub fn execute(ctx: Context<ExecuteState> ,dapp_id:u128
        ,tx_sender:String,_message:C3SolMessage)-> Result<()>{

        //require!(_message.data.len() >0,C3CallerErros::DataisEmpty);
        require!(!ctx.accounts.pause.is_paused,C3CallerErros::ContractIsPaused);
        require!(ctx.accounts.target_program.key() == _message.data.program_id,C3CallerErros::TargetProgramIdMismatch);
       

       let mut accounts = Vec::with_capacity(_message.data.accounts.len());

       for acc in _message.data.accounts.iter(){
        accounts.push( AccountMeta::from(acc));
           
       }
       let ix = Instruction{
        program_id: _message.data.program_id,
        accounts: accounts,
        data: _message.data.data.clone()
       };

       let account_infos = vec![
            ctx.accounts.target_program.clone(),
            ctx.accounts.signer.to_account_info(),
        ];
       let resutl = invoke(&ix, &account_infos);
        emit!(LogExecCall{
            dapp_id:dapp_id,
            to:_message.to.clone(),
            uuid:_message.uuid,
            from_chain_id:_message.from_chain_id,
            source_tx:_message.source_tx,
            data:_message.data.data.clone(),
            success:true,
            reason:Vec::new()

        });

        match resutl {
            Ok(_) => {
                //save uiid 
                ctx.accounts.c3_uuid.uuid_2_nonce.push(_message.uuid);
            },
            Err(_) => {
                emit!(LogFallbackCall{
                   dapp_id:dapp_id,
                   to:_message.to,
                   data:_message.data.data,
                   uuid:_message.uuid,
                   reasons:Vec::new()
                }
                    
                )

            },
        }
        Ok(())
    }
    #[access_control(check_owner(&ctx))]
    pub fn cfallback(ctx: Context<ExecuteState>, _dapp_id:u128, _tx_sender:Pubkey, _message:C3SolMessage)-> Result<()>{
    
       // require!(_message.data.len()>0,C3CallerErros::DataisEmpty);
        require!(!ctx.accounts.pause.is_paused,C3CallerErros::ContractIsPaused);
      

        let mut accounts = Vec::with_capacity(_message.data.accounts.len());

        for acc in _message.data.accounts.iter(){
         accounts.push( AccountMeta::from(acc));
            
        }
        let ix = Instruction{
         program_id: _message.data.program_id,
         accounts: accounts,
         data: _message.data.data.clone()
        };
 
        let account_infos = vec![
             ctx.accounts.target_program.clone(),
             ctx.accounts.signer.to_account_info(),
         ];
        let _resutl = invoke(&ix, &account_infos).unwrap();

        ctx.accounts.c3_uuid.uuid_2_nonce.push(_message.uuid);

    emit!(LogExecFallback{
        dapp_id:_dapp_id,
        to:_message.to,
        uuid:_message.uuid,
        from_chain_id:_message.from_chain_id,
        source_tx:_message.source_tx,
        data:_message.data.data,
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

fn check_owner(ctx: &Context<ExecuteState>)->Result<()>{
    require_keys_eq!(
        ctx.accounts.signer.key(),ctx.accounts.owner.onwer_key,C3CallerErros::NotOwner
    );

    
    Ok(())
}
#[event_cpi]
#[derive(Accounts)]
pub struct InitC3Caller<'info> {
    #[account(init,
    payer = signer,
    space= size_of::<C3UUIDKeeper>()+1024,
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
    #[account(init,
    payer = signer,
    space = 32+8,
    seeds = [OWNER_KEY_SEED],
    bump)]
    pub owner:Account<'info,OwnerKey>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program:Program<'info,System>,

}

#[event_cpi]
#[derive(Accounts)]
pub struct C3CallerState<'info>{
    #[account()]
    pub pause:Account<'info,Pause>,
    #[account()]
    pub owner:Account<'info,OwnerKey>,
    #[account(mut)]
    pub c3_uuid : Account<'info, C3UUIDKeeper>,
    #[account(mut)]
    pub signer: Signer<'info>,
}



#[derive(Accounts)]
pub struct SetPause<'info>{
    #[account(mut,seeds=[PAUSE_SEED], bump)]
    pub pause:Account<'info,Pause>,
    signer:Signer<'info>,

}

#[derive(Accounts)]
pub struct UpdateOwner<'info>{
    #[account(mut,seeds=[OWNER_KEY_SEED], bump)]
    owner:Account<'info,OwnerKey>,
    signer:Signer<'info>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct ExecuteState<'info>{
    #[account()]
    pub pause:Account<'info,Pause>,
    #[account()]
    pub owner:Account<'info,OwnerKey>,
    #[account(mut)]
    pub c3_uuid : Account<'info, C3UUIDKeeper>,
    #[account(mut)]
    pub signer: Signer<'info>,
     /// CHECK: This is not dangerous because we are verifying that this is the correct program ID in the function
    pub target_program: AccountInfo<'info>,
}




