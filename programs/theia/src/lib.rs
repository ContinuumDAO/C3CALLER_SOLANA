use anchor_lang::prelude::*;

declare_id!("7AQNimfj3jURLjQzphNHySCBceLaFyAqtF2QcngKS17y");

#[program]
pub mod theia {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
