use anchor_lang::prelude::*;

declare_id!("6zEAEfszDLaJ463PVDz8raSYgGQR74gv4TTJh4BLJM4C");

#[program]
pub mod theia_uuid_keeper {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
