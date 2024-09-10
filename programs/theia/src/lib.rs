use anchor_lang::prelude::*;

mod events;
pub use events::*;

declare_id!("7AQNimfj3jURLjQzphNHySCBceLaFyAqtF2QcngKS17y");

#[program]
pub mod theia {
   

    pub fn initialize(
        ctx: Context<Initialize>,
        w_native: Pubkey,
        uuid_keeper: Pubkey,
        theia_config: Pubkey,
        fee_manager: Pubkey,
        gov: Pubkey,
        c3caller_proxy: Pubkey,
        tx_sender: Pubkey,
        dapp_id: u64,
    ) -> Result<()> {
        let router = &mut ctx.accounts.router;
        router.w_native = w_native;
        router.uuid_keeper = uuid_keeper;
        router.theia_config = theia_config;
        router.fee_manager = fee_manager;
        router.gov = gov;
        router.c3caller_proxy = c3caller_proxy;
        router.tx_senders.push(tx_sender);
        router.dapp_id = dapp_id;
        router.bump = *ctx.bumps.get("router").unwrap();
        Ok(())
    }

    pub fn change_uuid_keeper(ctx: Context<ChangeConfig>, uuid_keeper: Pubkey) -> Result<()> {
        let router = &mut ctx.accounts.router;
        router.uuid_keeper = uuid_keeper;
        emit!(LogChangeUUIDKeeper { uuid_keeper });
        Ok(())
    }

    pub fn change_theia_config(ctx: Context<ChangeConfig>, theia_config: Pubkey) -> Result<()> {
        let router = &mut ctx.accounts.router;
        router.theia_config = theia_config;
        emit!(LogChangeTheiaConfig { theia_config });
        Ok(())
    }

    pub fn change_fee_manager(ctx: Context<ChangeConfig>, fee_manager: Pubkey) -> Result<()> {
        let router = &mut ctx.accounts.router;
        router.fee_manager = fee_manager;
        emit!(LogChangeFeeManager { fee_manager });
        Ok(())
    }

    pub fn set_minter(ctx: Context<SetMinter>, token: Pubkey, auth: Pubkey) -> Result<()> {
        // Implement set_minter logic here
        Ok(())
    }

    pub fn apply_minter(ctx: Context<ApplyMinter>, token: Pubkey) -> Result<()> {
        // Implement apply_minter logic here
        Ok(())
    }

    pub fn revoke_minter(ctx: Context<RevokeMinter>, token: Pubkey, auth: Pubkey) -> Result<()> {
        // Implement revoke_minter logic here
        Ok(())
    }

    pub fn get_liquidity(ctx: Context<GetLiquidity>, token: Pubkey) -> Result<(u64, u8)> {
        let (liquidity, decimals) = ctx.accounts.router.get_liquidity(token)?;
        Ok((liquidity, decimals))
    }

    pub fn query_liquidity_fee_rate(ctx: Context<QueryLiquidityFeeRate>, theia_token: Pubkey, amount: u64) -> Result<u64> {
        let router = &ctx.accounts.router;
        let fee_rate = router.query_liquidity_fee_rate(theia_token, amount)?;
        let base_fee = router.get_base_liquidity_fee(theia_token)?;
        let fee = (base_fee * fee_rate) / 1000;
        Ok(fee)
    }

    // Add other instruction handlers here...
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + TheiaRouter::MAX_SIZE)]
    pub router: Account<'info, TheiaRouter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeConfig<'info> {
    #[account(mut, has_one = gov)]
    pub router: Account<'info, TheiaRouter>,
    pub gov: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetMinter<'info> {
    #[account(mut, has_one = gov)]
    pub router: Account<'info, TheiaRouter>,
    pub gov: Signer<'info>,
    /// CHECK: This account is checked in the instruction
    pub token: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ApplyMinter<'info> {
    #[account(mut, has_one = gov)]
    pub router: Account<'info, TheiaRouter>,
    pub gov: Signer<'info>,
    /// CHECK: This account is checked in the instruction
    pub token: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RevokeMinter<'info> {
    #[account(mut, has_one = gov)]
    pub router: Account<'info, TheiaRouter>,
    pub gov: Signer<'info>,
    /// CHECK: This account is checked in the instruction
    pub token: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetLiquidity<'info> {
    pub router: Account<'info, TheiaRouter>,
    /// CHECK: This account is checked in the instruction
    pub token: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct QueryLiquidityFeeRate<'info> {
    pub router: Account<'info, TheiaRouter>,
    /// CHECK: This account is checked in the instruction
    pub theia_token: AccountInfo<'info>,
}


#[error_code]
pub enum TheiaRouterError {
    #[msg("Invalid input")]
    InvalidInput,
    // Add other error codes here...
}



