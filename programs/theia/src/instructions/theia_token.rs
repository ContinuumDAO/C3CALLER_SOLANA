
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, 
        Metadata as Metaplex,
    },
};
#[derive(Accounts)]
pub struct CreateTheiaToken{
    #[account(mut)]
    pub payer:Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = token::Mint::LEN,
        seeds = [b"theia_token"],
        bump,
        mint::authority = mint,
        mint::decimals = 9,
    )]
    pub mint:Account<'info,token::Mint>,
   /// CHECK: this is the metadata account for the token
    pub metadata:UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metaplex>,
  
}

#[derive(Accounts)]
pub struct MintTheiaToken{
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"theia_token"],
        bump,
        mint::authority = mint,
    )]
    pub mint: Account<'info, token::Mint>,
    #[account(
        init_if_needed,
        payer = payer, 
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub destination:Account<'info,TokenAccount>,
    pub rent:Sysvar<'info,Rent>, 
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>,
    
}

impl CreateTheiaToken<'_>{  
    pub fn apply(ctx: &mut Context<CreateTheiaToken>)->Result<()>{
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::CreateMint {
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );
        token::create_mint(cpi_context, decimals, ctx.accounts.payer.key, Some(ctx.accounts.payer.key))?;

}

}

impl MintTheiaToken<'_>{
    pub fn apply(ctx: &mut Context<MintTheiaToken>)->Result<()>{
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

}