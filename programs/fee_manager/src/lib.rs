use anchor_lang::prelude::*;
pub mod events;
use events::*;

declare_id!("2rxx7u9C5D5bMdjYmUz6LGkkfe6uwd2RPmgTVuidDW3B");

#[program]
pub mod fee_manager {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        gov: Pubkey,
        c3caller_proxy: Pubkey,
        tx_sender: Pubkey,
        dapp_id: u64,
    ) -> Result<()> {
       
        Ok(())
    }

    // pub fn add_fee_token(ctx: Context<AddFeeToken>, fee_token: Pubkey) -> Result<()> {
    //     let fee_manager = &mut ctx.accounts.fee_manager;
    //     require!(!fee_manager.fee_token_list.contains(&fee_token), FeeManagerError::TokenAlreadyExists);

    //     fee_manager.fee_token_list.push(fee_token);
    //     fee_manager.fee_token_index_map.insert(fee_token, fee_manager.fee_token_list.len() as u64);

    //     emit!(AddFeeTokenEvent {
    //         fee_token,
    //     });

    //     Ok(())
    // }

    // pub fn del_fee_token(ctx: Context<DelFeeToken>, fee_token: Pubkey) -> Result<()> {
    //     let fee_manager = &mut ctx.accounts.fee_manager;
    //     require!(fee_manager.fee_token_index_map.contains_key(&fee_token), FeeManagerError::TokenNotExist);

    //     let index = fee_manager.fee_token_index_map[&fee_token] as usize - 1;
    //     let len = fee_manager.fee_token_list.len();

    //     if index == len - 1 {
    //         fee_manager.fee_token_list.pop();
    //     } else {
    //         let last_token = fee_manager.fee_token_list[len - 1];
    //         fee_manager.fee_token_list.pop();
    //         fee_manager.fee_token_list[index] = last_token;
    //         fee_manager.fee_token_index_map.insert(last_token, (index + 1) as u64);
    //     }

    //     fee_manager.fee_token_index_map.remove(&fee_token);

    //     emit!(DelFeeTokenEvent {
    //         fee_token,
    //     });

    //     Ok(())
    // }

    // pub fn set_fee_config(
    //     ctx: Context<SetFeeConfig>,
    //     src_chain_id: u64,
    //     dst_chain_id: u64,
    //     pay_from: u64,
    //     fee_tokens: Vec<Pubkey>,
    //     fees: Vec<u64>,
    // ) -> Result<()> {
    //     let fee_manager = &mut ctx.accounts.fee_manager;
    //     require!(src_chain_id > 0 || dst_chain_id > 0, FeeManagerError::ChainIdEmpty);
    //     require!(
    //         pay_from == fee_manager.from_chain_pay || pay_from == fee_manager.to_chain_pay,
    //         FeeManagerError::InvalidPayFrom
    //     );
    //     require!(fee_tokens.len() == fees.len(), FeeManagerError::InvalidListSize);

    //     for (index, fee_token) in fee_tokens.iter().enumerate() {
    //         require!(
    //             fee_manager.fee_token_index_map.contains_key(fee_token),
    //             FeeManagerError::TokenNotExist
    //         );

    //         if pay_from == fee_manager.from_chain_pay {
    //             fee_manager
    //                 .from_fee_configs
    //                 .entry(ctx.accounts.clock.slot)
    //                 .or_default()
    //                 .insert(*fee_token, fees[index]);
    //         } else if pay_from == fee_manager.to_chain_pay {
    //             fee_manager
    //                 .to_fee_configs
    //                 .entry(dst_chain_id)
    //                 .or_default()
    //                 .insert(*fee_token, fees[index]);
    //         }

    //         emit!(SetFeeTokenEvent {
    //             fee_token: *fee_token,
    //             src_chain_id,
    //             dst_chain_id,
    //             fee: fees[index],
    //             pay_from,
    //         });
    //     }

    //     Ok(())
    // }

    // pub fn set_liq_base_fee(ctx: Context<SetLiqBaseFee>, fee_token: Pubkey, base_fee: u64) -> Result<()> {
    //     let fee_manager = &mut ctx.accounts.fee_manager;
    //     fee_manager.liq_base_fee_configs.insert(fee_token, base_fee);

    //     emit!(SetLiqFeeEvent {
    //         fee_token,
    //         fee: base_fee,
    //     });

    //     Ok(())
    // }

    // pub fn withdraw_fee(ctx: Context<WithdrawFee>, fee_token: Pubkey, amount: u64) -> Result<()> {
    //     let fee_manager = &mut ctx.accounts.fee_manager;
    //     let token_account = &ctx.accounts.token_account;
    //     let recipient = &ctx.accounts.recipient;

    //     let transfer_amount = amount.min(token_account.amount);

    //     let seeds = &[
    //         fee_manager.to_account_info().key.as_ref(),
    //         &[fee_manager.bump],
    //     ];
    //     let signer = &[&seeds[..]];

    //     let cpi_accounts = Transfer {
    //         from: token_account.to_account_info(),
    //         to: recipient.to_account_info(),
    //         authority: fee_manager.to_account_info(),
    //     };
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    //     //token::transfer(cpi_ctx, transfer_amount)?;

    //     emit!(WithdrawalEvent {
    //         fee_token,
    //         recipient: *recipient.key,
    //         amount: transfer_amount,
    //     });

    //     Ok(())
    // }

    // pub fn get_fee(ctx: Context<GetFee>, from_chain_id: u64, to_chain_id: u64, fee_token: Pubkey) -> Result<u64> {
    //     let fee_manager = &ctx.accounts.fee_manager;
    //     require!(from_chain_id > 0 || to_chain_id > 0, FeeManagerError::InvalidChainID);

    //     let mut fee = fee_manager.get_from_chain_fee(from_chain_id, fee_token);
    //     if fee == 0 {
    //         fee = fee_manager.get_to_chain_fee(to_chain_id, fee_token);
    //     }

    //     Ok(fee)
    // }

    // pub fn get_liquidity_fee(
    //     ctx: Context<GetLiquidityFee>,
    //     fee_token: Pubkey,
    //     from_chain_id: u64,
    //     to_chain_id: u64,
    //     liquidity: u64,
    //     amount: u64,
    //     underlying: bool,
    // ) -> Result<u64> {
    //     let fee_manager = &ctx.accounts.fee_manager;
    //     let base_fee = fee_manager.get_fee(from_chain_id, to_chain_id, fee_token);

    //     if base_fee == 0 {
    //         return Ok(0);
    //     }

    //     let fee_factor = if underlying {
    //         fee_manager.get_fee_factor(liquidity, amount)
    //     } else {
    //         1000
    //     };

    //     Ok((base_fee * fee_factor) / 1000)
    // }

    // pub fn get_liquidity_fee_factor(ctx: Context<GetLiquidityFeeFactor>, liquidity: u64, amount: u64) -> Result<u64> {
    //     let fee_manager = &ctx.accounts.fee_manager;
    //     Ok(fee_manager.get_fee_factor(liquidity, amount))
    // }

    // pub fn set_fee_token_params(ctx: Context<SetFeeTokenParams>, fee_token: Pubkey, params: FeeParams) -> Result<()> {
    //     let fee_manager = &mut ctx.accounts.fee_manager;
    //     fee_manager.fee_params.insert(fee_token, params);
    //     Ok(())
    // }

    // pub fn get_fee_token_params(ctx: Context<GetFeeTokenParams>, fee_token: Pubkey) -> Result<FeeParams> {
    //     let fee_manager = &ctx.accounts.fee_manager;
    //     fee_manager.fee_params.get(&fee_token).cloned().ok_or(FeeManagerError::TokenNotExist.into())
    // }

    // pub fn get_gas_fee(ctx: Context<GetGasFee>, to_chain_id: u64, fee_token: Pubkey) -> Result<u64> {
    //     let fee_manager = &ctx.accounts.fee_manager;
    //     let params = fee_manager.fee_params.get(&fee_token).ok_or(FeeManagerError::TokenNotExist)?;

    //     if params.base_price == 0 {
    //         return Ok(0);
    //     }

    //     if to_chain_id == 1 {
    //         // Simulating Ethereum gas price check. In Solana, you might want to use a different mechanism.
    //         let gas_price = Clock::get()?.unix_timestamp as u64; // Using unix_timestamp as a stand-in for gas price

    //         if gas_price < params.low_gas {
    //             Ok(params.base_price * params.low_gas_fee)
    //         } else if gas_price < params.normal_gas {
    //             Ok(params.base_price * params.normal_gas_fee)
    //         } else if gas_price < params.high_gas {
    //             Ok(params.base_price * params.high_gas_fee)
    //         } else {
    //             Ok(params.base_price * params.very_high_gas_fee)
    //         }
    //     } else {
    //         Ok(0) // only bother with Ethereum gas fees
    //     }
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct AddFeeToken<'info> {
//     #[account(mut, has_one = gov)]
//     pub fee_manager: Account<'info, FeeManager>,
//     pub gov: Signer<'info>,
// }

// #[derive(Accounts)]
// pub struct DelFeeToken<'info> {
//     #[account(mut, has_one = gov)]
//     pub fee_manager: Account<'info, FeeManager>,
//     pub gov: Signer<'info>,
// }

// #[derive(Accounts)]
// pub struct SetFeeConfig<'info> {
//     #[account(mut, has_one = gov)]
//     pub fee_manager: Account<'info, FeeManager>,
//     pub gov: Signer<'info>,
//     pub clock: Sysvar<'info, Clock>,
// }

// #[derive(Accounts)]
// pub struct SetLiqBaseFee<'info> {
//     #[account(mut, has_one = gov)]
//     pub fee_manager: Account<'info, FeeManager>,
//     pub gov: Signer<'info>,
// }

// #[derive(Accounts)]
// pub struct WithdrawFee<'info> {
//     #[account(mut, has_one = gov)]
//     pub fee_manager: Account<'info, FeeManager>,
//     pub gov: Signer<'info>,
//     #[account(mut)]
//     pub token_account: Account<'info, anchor_spl::token::TokenAccount>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub recipient: AccountInfo<'info>,
//     pub token_program: Program<'info, anchor_spl::token::Token>,
// }

// #[derive(Accounts)]
// pub struct GetFee<'info> {
//     pub fee_manager: Account<'info, FeeManager>,
// }

// #[derive(Accounts)]
// pub struct GetLiquidityFee<'info> {
//     pub fee_manager: Account<'info, FeeManager>,
// }

// #[derive(Accounts)]
// pub struct GetLiquidityFeeFactor<'info> {
//     pub fee_manager: Account<'info, FeeManager>,
// }

// #[derive(Accounts)]
// pub struct SetFeeTokenParams<'info> {
//     #[account(mut, has_one = gov)]
//     pub fee_manager: Account<'info, FeeManager>,
//     pub gov: Signer<'info>,
// }

// #[derive(Accounts)]
// pub struct GetFeeTokenParams<'info> {
//     pub fee_manager: Account<'info, FeeManager>,
// }

// #[derive(Accounts)]
// pub struct GetGasFee<'info> {
//     pub fee_manager: Account<'info, FeeManager>,
//     pub clock: Sysvar<'info, Clock>,
// }

// #[account]
// pub struct FeeManager {
//     pub gov: Pubkey,
//     pub c3caller_proxy: Pubkey,
//     pub dapp_id: u64,
//     pub tx_senders: Vec<Pubkey>,
//     pub fee_token_list: Vec<Pubkey>,
//     pub fee_token_index_map: HashMap<Pubkey, u64>,
//     pub from_fee_configs: HashMap<u64, HashMap<Pubkey, u64>>,
//     pub to_fee_configs: HashMap<u64, HashMap<Pubkey, u64>>,
//     pub liq_base_fee_configs: HashMap<Pubkey, u64>,
//     pub from_chain_pay: u64,
//     pub to_chain_pay: u64,
//     pub fee_params: HashMap<Pubkey, FeeParams>,
//     pub bump: u8,
// }

// impl FeeManager {
//     pub fn get_from_chain_fee(&self, from_chain_id: u64, fee_token: Pubkey) -> u64 {
//         self.from_fee_configs
//             .get(&from_chain_id)
//             .and_then(|config| config.get(&fee_token))
//             .cloned()
//             .unwrap_or(0)
//     }

//     pub fn get_to_chain_fee(&self, to_chain_id: u64, fee_token: Pubkey) -> u64 {
//         self.to_fee_configs
//             .get(&to_chain_id)
//             .and_then(|config| config.get(&fee_token))
//             .cloned()
//             .unwrap_or(0)
//     }

//     pub fn get_fee(&self, from_chain_id: u64, to_chain_id: u64, fee_token: Pubkey) -> u64 {
//         let fee = self.get_from_chain_fee(from_chain_id, fee_token);
//         if fee == 0 {
//             self.get_to_chain_fee(to_chain_id, fee_token)
//         } else {
//             fee
//         }
//     }

//     pub fn get_fee_factor(&self, liquidity: u64, amount: u64) -> u64 {
//         if liquidity == 0 {
//             return 0;
//         }
//         if amount > liquidity {
//             return 0;
//         }

//         // fee is 20 times higher to use all of the available liquidity
//         let fee_factor = [
//             1000, 1048, 1190, 1428, 1760, 2188, 2710, 3328, 4040, 4848,
//             5750, 6748, 7840, 9028, 10310, 11688, 13160, 14728, 16390, 18148, 20000
//         ];

//         let index = (100 * amount) / liquidity;

//         if index > 80 {
//             fee_factor[(index - 80) as usize]
//         } else {
//             1000
//         }
//     }
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
// #[derive(InitSpace)]
// pub struct FeeParams {
//     pub base_price: u64,
//     pub low_gas: u64,
//     pub normal_gas: u64,
//     pub high_gas: u64,
//     pub very_high_gas: u64,
//     pub low_gas_fee: u64,
//     pub normal_gas_fee: u64,
//     pub high_gas_fee: u64,
//     pub very_high_gas_fee: u64,
// }

#[error_code]
pub enum FeeManagerError {
    #[msg("Token already exists")]
    TokenAlreadyExists,
    #[msg("Token does not exist")]
    TokenNotExist,
    #[msg("Chain ID empty")]
    ChainIdEmpty,
    #[msg("Invalid pay from value")]
    InvalidPayFrom,
    #[msg("Invalid list size")]
    InvalidListSize,
    #[msg("Invalid chain ID")]
    InvalidChainID,
}

