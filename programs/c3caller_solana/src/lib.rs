use anchor_lang::prelude::*;
mod events;
declare_id!("2hFPiv6jk5sEF6gai3v1q76oUQXPnghVNJF3GqwdN1hh");

#[program]
pub mod c3caller_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
