use anchor_lang::prelude::*;


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
    pub system_program: Program<'info, System>,
}


impl  TheiaCrossEvm<'_> { 


    pub fn apply(&mut self,)->Result<()>{

        Ok(())
    }
    
}

