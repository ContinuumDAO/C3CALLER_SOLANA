mod states;
mod events;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use state::*;
use instructions::*;

declare_id!("2HJ1fdTUy1itAGc4XYsoygZQJi3tTqHoDUfx8n3dGD1J");

#[program]
pub mod theia_router_config {
    use super::*;

    pub fn get_token_config(ctx: Context<GetTokenConfig>,params:GetTokenConfigParams)-> Result<TokenConfig>{
            ctx.accounts.apply(&params)
    }
    pub fn set_token_config(ctx:Context<SetTokenConfig>,params:SetTokenConfigParams )-> Result<()>{
        ctx.accounts.apply(&params)
    }

    pub fn get_token_config_if_exist(ctx: Context<GetTokenConfig>,params:GetTokenConfigParams)->Result<(TokenConfig,TokenConfig)>{
        let c =  ctx.accounts.apply(&params)?;
        let tc =  ctx.accounts.apply(&params)?;
        Ok((c,tc))
    }

}









