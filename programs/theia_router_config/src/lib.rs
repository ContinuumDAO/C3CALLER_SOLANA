mod states;
mod events;

use anchor_lang::prelude::*;
use states::{TokenConfig, TokenIDMap, TokenIdList, TokenInfo};

declare_id!("2HJ1fdTUy1itAGc4XYsoygZQJi3tTqHoDUfx8n3dGD1J");

#[program]
pub mod theia_router_config {
    use events::LogSetTokenConfig;

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        gov: Pubkey,
        c3caller_proxy: Pubkey,
        tx_sender: Pubkey,
        dapp_id: u64,
    ) -> Result<()> {
        

        Ok(())
    }

    pub fn get_token_config(ctx: Context<GetTokenConfig>,token_id:String,chain_id:u64)-> Result<TokenConfig>{
        let token_config = &ctx.accounts.token_config;
        Ok(TokenConfig { chain_id: token_config.chain_id.clone(), decimals: token_config.decimals.clone(), contract_address: token_config.contract_address.clone(), contract_version: token_config.contract_version.clone(), router_contract: token_config.router_contract.clone(), extra: token_config.extra.clone() })
    }

    pub fn set_token_config(ctx:Context<SetTokenConfig>, token_id:String,chain_id:u64,token_addr:String,decimals:u8,contract_version:u64, routeur_contract:String,underlying:String)->Result<()>{



        ctx.accounts.token_config.chain_id = chain_id.clone();
        ctx.accounts.token_config.decimals = decimals.clone();
        ctx.accounts.token_config.contract_version = contract_version.clone();
        ctx.accounts.token_config.contract_address = token_addr.clone();
        ctx.accounts.token_config.router_contract = routeur_contract.clone();
        ctx.accounts.token_config.extra = underlying;
            
        ctx.accounts.token_ids_map.is_valid = true;
        ctx.accounts.token_list.token_ids.push(token_id.clone());


        emit!(LogSetTokenConfig{
             chain_id: chain_id,
            decimals: decimals,
            contract_version: contract_version,
            token_id: token_id,
            contract_address:token_addr ,
            router_contract: routeur_contract,
            extra: "".to_string() });

        Ok(())


    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
   // #[account(init, payer = user, space = 8 + TheiaRouterConfig::MAX_SIZE)]
  //  pub config: Account<'info, TheiaRouterConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(token_id:String, chain_id:u64)]
pub struct  SetTokenConfig<'info>{

    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(
        init,
        payer = signer,
        space =  8 + TokenConfig::INIT_SPACE,
        seeds = [token_id.as_bytes(), &chain_id.to_le_bytes()],
        bump,
    )]
    pub token_config:Account<'info,TokenConfig>,

    #[account(
        init,
        payer = signer,
        space = 8+ TokenIDMap::INIT_SPACE,
        seeds = [&chain_id.to_be_bytes()],
        bump,
    )]
    pub token_ids_map:Account<'info,TokenIDMap>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8+ 1024, // todo fix space
        seeds = [b"token list"],
        bump,

    )]
    pub token_list:Account<'info,TokenIdList>,

    pub system_program:Program<'info,System>

}

#[derive(Accounts)]
#[instruction(token_id: String, chain_id: u64)]
pub struct GetTokenConfig<'info> {
    #[account(
        seeds = [token_id.as_bytes(), &chain_id.to_le_bytes()],
        bump,
    )]
    pub token_config: Account<'info, TokenConfig>,
}







