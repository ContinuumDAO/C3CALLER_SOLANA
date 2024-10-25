use anchor_lang::prelude::*;
pub mod events;
pub mod state;
pub mod instructions;
use events::*;


declare_id!("CSymf8F2JmZ3Hk2wn42SfxfXJiPgs9u5529vVNBg5EXg");

#[program]
pub mod fee_manager {
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


    pub  fn set_fee_config(ctx: Context<SetFromFeeConfig>,src_chain_id:u64,
         dst_chain_id:u64,
          pay_from:u8, addresses:Vec<String>, fee:Vec<u64>)->Result<()>{


        

            
          

            for i in 0 .. fee.len()  {

                if pay_from == 1 {
                    ctx.accounts.fee_config.fee = fee[i];
                }else if pay_from == 2 {
                    ctx.accounts.fee_config.fee = fee[i];

                }
                
            }
      
    
        Ok(())

    }

    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}






#[error_code]
pub enum FeeManagerError {
    #[msg("Token already exists")]
    TokenAlreadyExists,
    #[msg("Token does not exist")]
    TokenNotExist,
    #[msg("Chain ID empty")]
    ChainIdEmpty,
    #[msg("Invalid pay from value")]
    InvalidPayFrom,
    #[msg("Invalid list size")]
    InvalidListSize,
    #[msg("Invalid chain ID")]
    InvalidChainID,
}

