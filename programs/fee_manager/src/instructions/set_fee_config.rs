use anchor_lang::prelude::*;
use crate::state::{FromFeeConfig, ToFeeConfig};

#[derive(Accounts)]
#[instruction(params:FeeConfigParams)]
pub struct SetFeeConfig<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8+ FromFeeConfig::INIT_SPACE,
        seeds = [b"from_chain",params.fee_token.as_bytes()],
        bump

    )]
    pub from_fee_config:Account<'info,FromFeeConfig>,

    #[account(
        init,
        payer = signer,
        space = 8+ ToFeeConfig::INIT_SPACE,
        seeds = [&params.dst_chain_id.to_le_bytes(),params.fee_token.as_bytes()],
        bump

    )]
    pub to_fee_config:Account<'info,ToFeeConfig>,
    pub  system_program:Program<'info,System>
}

impl <'info> SetFeeConfig<'info> {


    pub fn apply(&mut self,params:FeeConfigParams)->Result<()>{
       self.from_fee_config.fee = params.from_fee;
       self.to_fee_config.fee = params.to_fee;
       Ok(())
    }
    
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct FeeConfigParams{
    pub dst_chain_id:u64,
    pub fee_token:String,
    pub from_fee:u64,
    pub to_fee:u64,
 
}
