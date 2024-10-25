use anchor_lang::prelude::*;
use fee_manager::instructions::GetFeeConfig;
use theia_uuid_keeper::cpi::accounts::GenerateUuid;
use theia_uuid_keeper::program::TheiaUuidKeeper;
use theia_uuid_keeper::CurrentNonce;
use theia_uuid_keeper::state::EvmData;
use c_3caller_solana::cpi::accounts::C3CallerState;
use c_3caller_solana::program::C3callerSolana;
use c_3caller_solana::states::Pause;
use c_3caller_solana::states::OwnerKey;
use c_3caller_solana::states::C3UUIDKeeper;
use theia_router_config::cpi::accounts::GetTokenConfig;
use theia_router_config::program::TheiaRouterConfig;
use theia_router_config::state::TokenConfig;
use fee_manager::cpi::accounts::GetFeeConfig;
use fee_manager::program::FeeManager;
use fee_manager::state::{ToFeeConfig,FromFeeConfig};


use crate::TokenInfo;


#[derive(Accounts)]
pub struct TheiaCrossEvm<'info>{
  
    pub theia_uuid_keeper: Program<'info, TheiaUuidKeeper>,
    /// CHECK: PDA checked in TheiaUuidKeepers
      #[account(mut)]
    pub uuid_nonce: UncheckedAccount<'info>,
    #[account(mut)]
    pub current_nonce: Account<'info, CurrentNonce>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub c3_caller: Program<'info, C3callerSolana>,
    #[account(mut)]
    pub c3_uuid: Account<'info, C3UUIDKeeper>,
    #[account(mut)]
    pub pause: Account<'info, Pause>,
    #[account(mut)]
    pub owner_key: Account<'info, OwnerKey>, 

    #[account()]
    pub theia_config:Program<'info,TheiaRouterConfig>,
    #[account()]
    pub token_config:Account<'info,TokenConfig>,
    #[account()]
    pub fee_manager:Program<'info,FeeManager>,
    #[account()]
    pub to_token_fee:Account<'info,ToFeeConfig>,
    #[account()]
    pub from_token_fee:Account<'info,FromFeeConfig>,
    pub system_program: Program<'info, System>,
}


impl  TheiaCrossEvm<'_> { 




    fn calc_and_pay(&mut self)->u64{

        let fee_cpi_ctx = CpiContext::new( self.fee_manager,
        GetFeeConfig{
            signer: self.payer,
            from_fee_config: self.from_token_fee,
            to_fee_config: self.to_token_fee,
        });

        fee_manager::cpi::get_fee(fee_cpi_ctx, params).unwrap().get()
    }
    fn get_rev_amount(&mut self)->u64{

         



        65_u64

    }
    fn get_uuid(&mut self)->[u8;32]{

        let uuid_cpi_ctx = CpiContext::new(self.theia_uuid_keeper.to_account_info(), GenerateUuid{
            uuid_nonce: self.uuid_nonce.to_account_info(),
            payer: self.payer.to_account_info(),
            current_nonce: self.current_nonce.to_account_info(),
            system_program: self.system_program.to_account_info(),
        });
        theia_uuid_keeper::cpi::gen_uuid_evm(uuid_cpi_ctx, EvmData {
            token: t.addr,
            from: self.payer.key().to_string(),
            amount: 0,
            receiver: params.receiver.clone(),
            to_chain_id: params.to_chain_id,
        }).unwrap().get()
            
    }


    pub  fn get_token_info(&mut self)->TokenInfo{

        let token_config_cpi_ctx = CpiContext::new(self.theia_config.to_account_info(),
        GetTokenConfig{
            token_config: self.token_config.to_account_info()
        });

        let (from_config,to_config) = theia_router_config::cpi::get_token_config_if_exist(token_config_cpi_ctx, params).unwrap().get();
        TokenInfo {
            addr: from_config.contract_address,
            decimals: from_config.decimals,
            to_chain_addr: to_config.contract_address,
            to_chain_decimals: to_config.decimals
             }
       
    }

    pub fn apply(&mut self,)->Result<()>{


        let  t = self.get_token_info();
       

        let recv_amount = params.amount;
       

        let uuid = self.get_uuid();

         let swap_fee = self.calc_and_pay();

         
         let token_info = TokenInfo {
             addr: "".to_string(), // Replace with actual token address
             decimals: 8, // Replace with actual decimals
             to_chain_addr: "".to_string(), // Replace with actual to_chain_addr
             to_chain_decimals: 8, // Replace with actual to_chain_decimals
         };
         let fee_token_info = TokenInfo {
             addr: "".to_string(), // Replace with actual fee token address
             decimals: 8, // Replace with actual fee token decimals
             to_chain_addr: "".to_string(), // Replace with actual fee token to_chain_addr
             to_chain_decimals: 8, // Replace with actual fee token to_chain_decimals
         };
         let data = gen_calldata(uuid, recv_amount, swap_fee, &params, &token_info, &fee_token_info);

        let ctx_caller = CpiContext::new(ctx.accounts.c3_caller.to_account_info(), C3CallerState {
            pause: ctx.accounts.pause.to_account_info(),
            owner: ctx.accounts.owner_key.to_account_info(),
            c3_uuid: ctx.accounts.c3_uuid.to_account_info(),
            signer: ctx.accounts.payer.to_account_info(),
        });

        let res_caller = c_3caller_solana::cpi::ccall(
            ctx_caller,
            40,  // dummy Dapp id 
            *ctx.program_id,
            params.target,
            params.to_chain_id.to_string(),
            data,
            Vec::new()
        );

        match res_caller {
            Ok(_) => msg!("cpi to c3call success"),
            Err(_) => return err!(errors::TheiaError::CcallCpiFailed),
        }
            

        let token = theia_router_config::cpi::token_config(ctx, params)?.get();



        //Result<Return<TokenConfig>>

        // match token {
        //     Ok(token_conf) => msg!(t),
        //     Err(_) => msg!(),
        // }
            
        

        
        

       

            
        
        

        
         //todo cpi into c3caller

         // then emit event
        emit!(
            LogTheiaCross{
                token: params.token_id,
                from: ctx.program_id.to_string(),
                swapout_id: uuid, 
                amount: recv_amount,
                from_chain_id: 900 ,// solana mainnet chain Id ,  
                to_chain_id: params.to_chain_id,
                fee: swap_fee, 
                fee_token: fee_token_info.addr,
                receiver: params.receiver,
            }
        );

        Ok(())
    }
    
}

