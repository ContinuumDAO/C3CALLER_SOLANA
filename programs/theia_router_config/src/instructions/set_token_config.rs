use anchor_lang::prelude::*;
use crate::TokenConfig;

#[derive(Accounts)]
#[instruction(params:SetTokenConfigParams)]
pub struct  SetTokenConfig<'info>{

    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(
        init,
        payer = signer,
        space =  8 + TokenConfig::INIT_SPACE,
        seeds = [TOKEN_CONFIG_SEED,params.token_id, params.chain_id],
        bump,
    )]
    pub token_config:Account<'info,TokenConfig>,
    pub system_program:Program<'info,System>

}

impl SetTokenConfig<'_> {

    pub fn apply(ctx: &mut Context<SetTokenConfig>, params: &SetTokenConfigParams)->Result<()>{

        ctx.accounts.token_config.chain_id = params.chain_id.clone();
        ctx.accounts.token_config.decimals = params.decimals.clone();
        ctx.accounts.token_config.contract_version = params.contract_version.clone();
        ctx.accounts.token_config.contract_address = params.token_addr.clone();
        ctx.accounts.token_config.router_contract = params.routeur_contract.clone();
        ctx.accounts.token_config.extra = params.underlying;
            

        emit!(LogSetTokenConfig{
            chain_id: params.chain_id,
            decimals: params.decimals,
            contract_version: params.contract_version,
            token_id: params.token_id,
            contract_address:params.token_addr ,
            router_contract: params.routeur_contract,
            extra: params.underlying });
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