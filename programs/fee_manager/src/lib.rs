use anchor_lang::prelude::*;
pub mod events;
pub mod state;
pub mod instructions;
use instructions::SetFeeConfig;



declare_id!("CSymf8F2JmZ3Hk2wn42SfxfXJiPgs9u5529vVNBg5EXg");

#[program]
pub mod fee_manager {
    use anchor_lang::prelude::Context;

    
    use crate::instructions::{FeeConfigParams, SetFeeConfig};
 
    pub fn init_fee_config(ctx: Context<SetFeeConfig>, params: FeeConfigParams) -> Result<()> {
        ctx.accounts.apply(params)
           
    }

    pub fn get_fee(ctx:Context<GetFeeConfig>,params:GetFeeConfigParams)->Result<u64>{
        ctx.accounts.apply(params)
    }

    
}



