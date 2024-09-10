use anchor_lang::prelude::*;
mod state;
use crate::state::*;

declare_id!("6zEAEfszDLaJ463PVDz8raSYgGQR74gv4TTJh4BLJM4C");

#[program]
pub mod theia_uuid_keeper {
    use anchor_lang::solana_program::keccak::hash;
    use state::NonEvmData;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn is_uuid_exist(ctx:Context<Initialize> ) -> Result<()>{
        Ok(())
    }

    pub fn is_completed(ctx:Context<CompleteSwap>)-> Result<()>{

            // retunn swap status of a swap
        Ok(())
    }
    pub fn revoke_swapin(ctx:Context<CompleteSwap>)->Result<()>{
            // set swap status to false
        Ok(())
    }
    pub fn register_uuid(ctx: Context<CompleteSwap>) ->Result<()>{

        // set swap status to true
        Ok(())
    }

    pub fn gen_uuid_non_evm(ctx: Context<ThieaKeeper>, data:NonEvmData)->Result<[u8;32]>{


        // onlyoperator can call this function
        // autoIncreasd swapoutoutNonce
        // check if the uuid account exist
        // set nonce value to account uui2Nonce 
        //then return  the value of the generated uuid
        let uuid = hash(
            
            &[
                &ctx.program_id.to_bytes()[..],
                &ctx.accounts.payer.key.to_bytes()[..],
                &data.token.as_bytes()[..],
                &data.from.as_bytes()[..],
                &data.receiver.as_bytes()[..],
                &data.amount.to_be_bytes()[..],
                &ctx.accounts.current_nonce.nonce.to_be_bytes()[..],
                & data.to_chain_id.as_bytes()[..],
                & data.call_data[..]
            ].concat()
        ).to_bytes();
        
        
        ctx.accounts.uuid_nonce.nonce = ctx.accounts.current_nonce.nonce;
        Ok(uuid)

    }

    // pub fn calc_caller_uuid(){
    //     // increment a nonce from the current nonce value
    //     // return encode  and hash with kecca256

        
    // }

    // pub fn calc_caller_encode(){

    //     // updated nonce local variable and increament the cureent by 1
    //     // encode


    // }
}

#[derive(Accounts)]
pub struct Initialize<'info>{


    #[account(
        init,
        payer = signer,
        space = 8 + CurrentNonce::INIT_SPACE,
        seeds = [],
        bump
    )]
    pub current_nonce:Account<'info,CurrentNonce>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>
}


#[derive(Accounts)]
#[instruction(uuid: String)]
pub struct CompleteSwap<'info>{
    #[account(mut)]
    pub payer:Signer<'info>,
    #[account(
        mut,
        seeds = [uuid.as_bytes().as_ref()],
        bump,
    )]
    pub swap: Account<'info,Swap>,

    #[account(mut)]
    pub current_nonce:Account<'info,CurrentNonce>

}

#[derive(Accounts)]
#[instruction(uuid:String)]
pub struct ThieaKeeper<'info>{
    #[account(mut)]
    pub payer:Signer<'info>,
    #[account(mut)]
    pub current_nonce:Account<'info,CurrentNonce>,
    #[account(mut)]
    pub uuid_nonce:Account<'info,Uuid2Nonce>,
}

impl CompleteSwap<'_>{

    pub fn apply(ctx: &mut Context<CompleteSwap>, uuid:&String)->Result<()>{
        
        Ok(())
    }

}



#[account]
#[derive(InitSpace)]
pub struct Swap{
    pub completed:bool,
}

#[account]
#[derive(InitSpace)]
pub struct Uuid2Nonce{
    pub nonce:u64,
}


#[account]
#[derive(InitSpace)]
pub struct CurrentNonce{
    pub nonce:u64
}
