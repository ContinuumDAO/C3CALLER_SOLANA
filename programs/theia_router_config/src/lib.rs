mod states;

use anchor_lang::prelude::*;
use states::TokenInfo;

declare_id!("2HJ1fdTUy1itAGc4XYsoygZQJi3tTqHoDUfx8n3dGD1J");

#[program]
pub mod theia_router_config {
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

    pub fn get_token_config(ctx: Context<Initialize>)->Result<()>{

        
Ok(())
    }

    pub fn set_token_config(ctx:Context<Initialize>)->Result<()>{



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
#[instruction(token_id:String, chain_id:u8)]
pub struct  SetTokenConfig<'info>{

    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        payer = user,
        space =  8 + TokenInfo::INIT_SPACE,
        seeds = [token_id.as_bytes(),&[chain_id]],
        bump,
    )]
    pub token_info:Account<'info,TokenInfo>,

    pub system_program:Program<'info,System>
}






