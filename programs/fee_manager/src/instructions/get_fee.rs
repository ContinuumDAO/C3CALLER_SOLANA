use anchor_lang::prelude::*;
use crate::state::{FromFeeConfig, ToFeeConfig};

#[derive(Accounts)]
#[instruction(params:GetFeeConfigParams)]
pub struct GetFeeConfig<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"from_chain",params.fee_token.as_bytes()],
        bump

    )]
    pub from_fee_config:Account<'info,FromFeeConfig>,

    #[account(
        seeds = [&params.dst_chain_id.to_le_bytes(),params.fee_token.as_bytes()],
        bump

    )]
    pub to_fee_config:Account<'info,ToFeeConfig>,
   
}

impl <'info> GetFeeConfig<'info> {


    pub fn apply(&mut self,params:GetConfigParams)->Result<u64>{
      let fee = self.from_fee_config.fee;
      if fee == 0{
          fee = self.to_fee_config.fee;
      }
      Ok(fee)
    }
    
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GetFeeConfigParams{
    pub dst_chain_id:u64,
    pub fee_token:String,
    
 
}