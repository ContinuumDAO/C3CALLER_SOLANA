use anchor_lang::prelude::*;
use crate::{events::LogSetTokenConfig, seeds::TOKEN_CONFIG_SEED, TokenConfig};

#[derive(Accounts)]
#[instruction(params:SetTokenConfigParams)]
pub struct  SetTokenConfig<'info>{

    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(
        init,
        payer = signer,
        space =  8 + TokenConfig::INIT_SPACE,
        seeds = [TOKEN_CONFIG_SEED, params.token_id.as_bytes(), &params.chain_id.to_le_bytes()],
        bump,
    )]
    pub token_config:Account<'info,TokenConfig>,
    pub system_program:Program<'info,System>

}

impl SetTokenConfig<'_> {

    pub fn apply(&mut self, params: &SetTokenConfigParams)->Result<()>{

        self.token_config.chain_id = params.chain_id;
        self.token_config.decimals = params.decimals;
        self.token_config.contract_version = params.contract_version;
        self.token_config.contract_address = params.token_addr.clone();
        self.token_config.router_contract = params.routeur_contract.clone();
        self.token_config.extra = params.underlying.clone();
            
        emit!(LogSetTokenConfig{
            chain_id: params.chain_id,
            decimals: params.decimals,
            contract_version: params.contract_version,
            token_id: params.token_id.clone(),
            contract_address:params.token_addr.clone() ,
            router_contract: params.routeur_contract.clone(),
            extra: params.underlying.clone() });
        Ok(())
    }
    
}



#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetTokenConfigParams{
    pub token_id:String,
    pub chain_id:u64,
    pub token_addr:String,
    pub decimals:u8,
    pub contract_version:u64,
    pub routeur_contract:String,
    pub underlying:String
    
}