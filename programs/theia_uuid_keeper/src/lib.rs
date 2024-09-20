use anchor_lang::prelude::*;
pub mod state;
use crate::state::*;


declare_id!("6zEAEfszDLaJ463PVDz8raSYgGQR74gv4TTJh4BLJM4C");

#[program]
pub mod theia_uuid_keeper {
    use anchor_lang::solana_program::keccak::hash;
    pub use state::NonEvmData;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        
        Ok(())
    }

   
    pub fn is_completed(ctx:Context<TheiaKeeper>)-> Result<bool>{
        Ok(ctx.accounts.uuid_nonce.completed)
    }
  
    pub fn register_uuid(ctx: Context<TheiaKeeper>) ->Result<()>{
        ctx.accounts.uuid_nonce.completed = true;   // set swap status to true
        Ok(())
    }

    pub fn set_nonce(ctx:Context<TheiaKeeper>, nonce:u64)->Result<()>{
        ctx.accounts.uuid_nonce.nonce = nonce;
        ctx.accounts.uuid_nonce.completed = false;
        Ok(())
    }
    pub fn gen_uuid_non_evm(ctx: Context<GenerateUuid>, data:NonEvmData)->Result<[u8;32]>{


        // onlyoperator can call this function
        // autoIncreasd swapoutoutNonce
        // check if the uuid account exist
        // set nonce value to account uui2Nonce 
        //then return  the value of the generated uuid
        let current_nonce = ctx.accounts.current_nonce.nonce + 1 ;
        let uuid = hash(
            
            &[
                &ctx.program_id.to_bytes()[..],
                &ctx.accounts.payer.key.to_bytes()[..],
                &data.token.as_bytes()[..],
                &data.from.as_bytes()[..],
                &data.receiver.as_bytes()[..],
                &data.amount.to_be_bytes()[..],
                &current_nonce.to_be_bytes()[..],
                & data.to_chain_id.as_bytes()[..],
                & data.call_data[..]
            ].concat()
        ).to_bytes(); 

        ctx.accounts.uuid_nonce.uuid = uuid;
        ctx.accounts.uuid_nonce.nonce = current_nonce;
        ctx.accounts.uuid_nonce.completed = false;
        
        ctx.accounts.current_nonce.nonce = current_nonce;
        Ok(uuid)

    }

    pub fn gen_uuid_evm(ctx:Context<GenerateUuid>, data:EvmData)->Result<[u8;32]>{
        let current_nonce = ctx.accounts.current_nonce.nonce + 1 ; 
        let uuid = hash(
        
            &[
                &ctx.program_id.to_bytes()[..],
                &ctx.accounts.payer.key.to_bytes()[..],
                &data.token.as_bytes()[..],
                &data.from.as_bytes()[..],
                &data.receiver.as_bytes()[..], 
                &data.amount.to_be_bytes()[..],
                &current_nonce.to_be_bytes()[..],
                &data.to_chain_id.as_bytes()[..],
            ].concat()
        ).to_bytes(); 

        ctx.accounts.uuid_nonce.uuid = uuid;
        ctx.accounts.uuid_nonce.nonce = current_nonce;
        ctx.accounts.uuid_nonce.completed = false;

        ctx.accounts.current_nonce.nonce = current_nonce;
        Ok(uuid)
    }
   
}

#[derive(Accounts)]
pub struct Initialize<'info>{


    #[account(
        init,
        payer = signer,
        space = 8 + CurrentNonce::INIT_SPACE,
        seeds = [b"current_nonce"],
        bump
    )]
    pub current_nonce:Account<'info,CurrentNonce>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>
}



#[derive(Accounts)]
pub struct GenerateUuid<'info>{
    #[account(
        init,
        payer = payer,
        space = 8 + Uuid2Nonce::INIT_SPACE,
        seeds = [b"uuid_nonce",(current_nonce.nonce + 1).to_be_bytes().as_ref()],
        bump
    )] 
    pub uuid_nonce:Account<'info,Uuid2Nonce>, 
    #[account(mut)] 
    pub payer:Signer<'info>,
    #[account(mut)]
    pub current_nonce:Account<'info,CurrentNonce>,
    pub system_program:Program<'info,System>
  
}
#[derive(Accounts)]
pub struct TheiaKeeper<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"uuid_nonce", uuid_nonce.nonce.to_be_bytes().as_ref()],
        bump
    )]
    pub uuid_nonce: Account<'info, Uuid2Nonce>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Uuid2Nonce{
    pub uuid:[u8;32],
    pub nonce:u64,
    pub completed:bool,
}

#[account]
#[derive(InitSpace)]
pub struct CurrentNonce{
    pub nonce:u64
}
