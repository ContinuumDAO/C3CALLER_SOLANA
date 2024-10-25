use anchor_lang::prelude::*;
mod events;
mod states;
mod errors;
mod instructions;
pub use events::*;
pub use states::*;


declare_id!("vBEVNQBYDoJSTf7U8zYjXoPQ85uFDXL25SuXb7ruE5n");

#[program]
pub mod theia {
    use super::*;


    pub fn initialize(
        ctx: Context<Initialize> ) -> Result<()> {
       
        Ok(())
    }

    


    pub fn theia_cross_non_evm(ctx: Context<TheiaCrossNonEvm>) -> Result<()>{

        
        //check if the params are valid

        //get _revAmount from getRevAmount()
         // generate the uuid by cpi into uuid keeper

         //  get swap fee  from _calcAndPay()

         // geneerate calldata and cpi into c3caller

         // then emit event
        Ok(())
    }

    pub fn theia_cross_evm(ctx: Context<TheiaCrossEvm>, params: CrossAuto) -> Result<()>{
        ctx.accounts.apply(params)
    }

//     pub fn theia_vault_auto(ctx: Context<TheiaVaultAuto>, params: VaultAuto) -> Result<()>{

     
         





//         Ok(())

//     }

  
// }

// pub fn _get_rev_amount(ctx: Context<TheiaVaultAuto>, params: VaultAuto) -> Result<()>{


//     // transfer native vs transfer

//     Ok(())

 }


 pub fn gen_calldata(uuid:[u8;32], recv_amount:u64, swap_fee:u64,tc:&CrossAuto, t:&TokenInfo,fee:&TokenInfo)->Vec<u8>{

    let to_amount = recv_amount;
    let liquidty_fee:u64 = 0;

    let func_sign_theia = "0x3a1f8688";//"theiaVaultAuto(bytes32,address,address,uint256,uint256,uint256,address,address)";

    let mut  call_data = Vec::new();
    call_data.extend_from_slice(func_sign_theia.as_bytes());
    call_data.extend_from_slice(&uuid);
    call_data.extend_from_slice(t.to_chain_addr.as_bytes());
    call_data.extend_from_slice(tc.receiver.as_bytes());
    call_data.extend_from_slice(&to_amount.to_le_bytes());
    call_data.extend_from_slice(&[t.to_chain_decimals]);
    call_data.extend_from_slice(&(liquidty_fee as u64).to_le_bytes());
    call_data.extend_from_slice(fee.to_chain_addr.as_bytes());
    call_data.extend_from_slice(t.addr.as_bytes());

    call_data
}
#[derive(Accounts)]
pub struct Initialize<'info> {
   

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TheiaCrossNonEvm<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}






// #[error_code]
// pub enum TheiaRouterError {
//     #[msg("Invalid input")]
//     InvalidInput,
//     // Add other error codes here...
// }



