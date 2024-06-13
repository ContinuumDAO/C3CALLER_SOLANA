use anchor_lang::prelude::*;
mod errors;
use crate::errors::C3CallerErros;
declare_id!("2hFPiv6jk5sEF6gai3v1q76oUQXPnghVNJF3GqwdN1hh");

#[program]
pub mod c3caller_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        require!(ctx.remaining_accounts.len()>0,C3CallerErros::DappIdisEmpty);
        Ok(())
    }

    pub fn c3call(ctx: Context<Initialize>,_dapp_id:u128,_caller:Pubkey,_to:String, _to_chain_id:String,_data:Vec<u8>,_extra:Vec<u8>)-> Result<()> {


        Ok(())
        
    }


}

#[derive(Accounts)]
pub struct Initialize {}
