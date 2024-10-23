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

    pub fn token_config(mut ctx: Context<GetTokenConfig>,params:GetTokenConfigParams)-> Result<TokenConfig>{
       GetTokenConfig::apply(&mut ctx, &params)
    }
    pub fn set_token_config(mut ctx:Context<SetTokenConfig>,params:SetTokenConfigParams )-> Result<()>{
        SetTokenConfig::apply(&mut ctx, &params)
    }

}









