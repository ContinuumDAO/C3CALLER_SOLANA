use anchor_lang::prelude::*;

declare_id!("VSo8f94a5JPj3W7CBmwbRpXFa78r75YjpLEK9WJpedm");

#[program]
pub mod govern_dapp {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        gov: Pubkey,
        c3caller_proxy: Pubkey,
         dapp_id: u64,
    ) -> Result<()> {
        let govern_state = &mut ctx.accounts.govern_state;
        govern_state.old_gov = gov;
        govern_state.new_gov = gov;
        govern_state.new_gov_effective_time = Clock::get()?.unix_timestamp as u64;
        govern_state.delay = 2 * 24 * 60 * 60; // 2 days in seconds
        govern_state.c3caller_proxy = c3caller_proxy;
        govern_state.dapp_id = dapp_id;


        Ok(())
    }

    pub fn change_gov(ctx: Context<ChangeGov>, new_gov: Pubkey) -> Result<()> {
        require!(new_gov != Pubkey::default(), GovernError::EmptyNewGov);

        let govern_state = &mut ctx.accounts.govern_state;
        let current_time = Clock::get()?.unix_timestamp as u64;

        govern_state.old_gov = govern_state.gov();
        govern_state.new_gov = new_gov;
        govern_state.new_gov_effective_time = current_time + govern_state.delay;

        emit!(LogChangeGov {
            old_gov: govern_state.old_gov,
            new_gov: govern_state.new_gov,
            effective_time: govern_state.new_gov_effective_time,
        });

        Ok(())
    }

    pub fn set_delay(ctx: Context<SetDelay>, new_delay: u64) -> Result<()> {
        let govern_state = &mut ctx.accounts.govern_state;
        govern_state.delay = new_delay;
        Ok(())
    }

    pub fn add_tx_sender(ctx: Context<UpdateTxSender>, tx_sender: Pubkey) -> Result<()> {
        
        let tx_sender_account = &mut ctx.accounts.tx_sender;    
        tx_sender_account.is_valid = true;
        emit!(LogTxSender {
            tx_sender,
            valid: true,
        });

        Ok(())
    }

    pub fn disable_tx_sender(ctx: Context<UpdateTxSender>, sender: Pubkey) -> Result<()> {
    
        let tx_sender_account = &mut ctx.accounts.tx_sender;    
        tx_sender_account.is_valid = false;
        emit!(LogTxSender {
            tx_sender: sender,
            valid: false,
        });

        Ok(())
    }

    pub fn get_tx_sender(ctx: Context<UpdateTxSender>,sender: Pubkey) -> Result<bool> {
        let tx_sender_account = &ctx.accounts.tx_sender;
        Ok(tx_sender_account.is_valid)
    }

    pub fn do_gov(ctx: Context<DoGov>, to: String, to_chain_id: String, data: Vec<u8>) -> Result<()> {
       // to do call C3call via cpi
        Ok(())
    }

    pub fn do_gov_broadcast(ctx: Context<DoGovBroadcast>, targets: Vec<String>, to_chain_ids: Vec<String>, data: Vec<u8>) -> Result<()> {
        require!(targets.len() == to_chain_ids.len(), GovernError::LengthMismatch);
        
        // to do c3broadcast via cpi
        Ok(())
    }

    //todo add gov check
}

#[derive(Accounts)]
#[instruction(sender: Pubkey)]  
pub struct Initialize<'info> {
    #[account(
        init, payer = signer, space = 8+ GovState::INIT_SPACE, seeds = [b"govern_state"], bump
    )]
    pub govern_state: Account<'info, GovState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeGov<'info> {
    #[account(mut, seeds = [b"govern_state"], bump)]
    pub govern_state: Account<'info, GovState>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetDelay<'info> {
    #[account(mut, seeds = [b"govern_state"], bump)]
    pub govern_state: Account<'info, GovState>,
    pub signer: Signer<'info>,
}



#[derive(Accounts)]
pub struct DoGov<'info> {
    #[account(mut, seeds = [b"govern_state"], bump)]
    pub govern_state: Account<'info, GovState>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct DoGovBroadcast<'info> {
    #[account(mut, seeds = [b"govern_state"], bump)]
    pub govern_state: Account<'info, GovState>,
    pub signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct GovState {
    pub old_gov: Pubkey,
    pub new_gov: Pubkey,
    pub new_gov_effective_time: u64,
    pub delay: u64,
    pub c3caller_proxy: Pubkey,
    pub dapp_id: u64,
  
}

impl GovState {
    pub fn gov(&self) -> Pubkey {
        let current_time = Clock::get().unwrap().unix_timestamp as u64;
        if current_time >= self.new_gov_effective_time {
            self.new_gov
        } else {
            self.old_gov
        }
    }

   
}

#[account]
#[derive(InitSpace)]
pub struct TxSender {
    pub is_valid: bool,
}

#[derive(Accounts)]
#[instruction(sender: Pubkey)]  
pub struct UpdateTxSender<'info> {
    #[account(
        mut,
        seeds = [b"tx_sender",sender.key().as_ref() ],
        bump,
    )]
    pub tx_sender: Account<'info, TxSender>,
    pub signer: Signer<'info>,
}

#[error_code]
pub enum GovernError {
    #[msg("New governor address cannot be empty")] 
    EmptyNewGov,
    #[msg("Length of targets and toChainIDs must match")]
    LengthMismatch,
}

#[event]
pub struct LogChangeGov {
    pub old_gov: Pubkey,
    pub new_gov: Pubkey,
    pub effective_time: u64,
}

#[event]
pub struct LogTxSender {
    pub tx_sender: Pubkey,
    pub valid: bool,
}

    