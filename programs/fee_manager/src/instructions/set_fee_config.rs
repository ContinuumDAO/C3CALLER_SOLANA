use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(chain_id:u64,token_addr:String)]
pub struct SetFeeConfig<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8+ FromFeeConfig::INIT_SPACE,
        seeds = [&chain_id.to_be_bytes(), token_addr.as_bytes()],
        bump

    )]
    pub from_fee_config:Account<'info,FromFeeConfig>,

    

    #[account(
        init,
        payer = signer,
        space = 8+ FromFeeConfig::INIT_SPACE,
        seeds = [&chain_id.to_be_bytes(), token_addr.as_bytes()],
        bump

    )]
    pub to_fee_config:Account<'info,ToFeeConfig>,
    pub  system_program:Program<'info,System>
}

