use anchor_lang::prelude::*;

declare_id!("2rxx7u9C5D5bMdjYmUz6LGkkfe6uwd2RPmgTVuidDW3B");

#[program]
pub mod fee_manager {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
