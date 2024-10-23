use anchor_lang::prelude::*;
use crate::{seeds::TOKEN_CONFIG_SEED, TokenConfig};

#[derive(Accounts)]
#[instruction(params:GetTokenConfigParams)]
pub struct  GetTokenConfig <'info>{
    #[account(
        seeds = [TOKEN_CONFIG_SEED, params.token_id.as_str().as_bytes(), &params.chain_id.to_le_bytes()],
        bump = 9,
    )]
    pub token_config:Account<'info,TokenConfig>,

}

impl GetTokenConfig<'_> {
    pub fn apply(ctx: &mut Context<GetTokenConfig>, params: &GetTokenConfigParams)->Result<TokenConfig>{
        let token_config = &ctx.accounts.token_config;
        
        Ok(TokenConfig { chain_id: token_config.chain_id, decimals: token_config.decimals, contract_address: token_config.contract_address.clone(), contract_version: token_config.contract_version, router_contract: token_config.router_contract.clone(), extra: token_config.extra.clone() })
    }
    
}



#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GetTokenConfigParams{
    pub token_id:String,
    pub chain_id:u64, 
}