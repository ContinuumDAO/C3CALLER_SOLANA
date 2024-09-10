use anchor_lang::prelude::*;

declare_id!("Gh327SmhfBRnfaYJwwC59h8vZs3psnAZWqQm1wXPuERq");

#[program]
pub mod theia_router_config {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        gov: Pubkey,
        c3caller_proxy: Pubkey,
        tx_sender: Pubkey,
        dapp_id: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.gov = gov;
        config.c3caller_proxy = c3caller_proxy;
        config.dapp_id = dapp_id;
        config.tx_senders.push(tx_sender);
        config.config_version = 1;
        config.config_role = hash(&b"CONFIG_ROLE"[..]).to_bytes();
        config.bump = *ctx.bumps.get("config").unwrap();

        config.grant_role(config.config_role, gov)?;
        config.grant_role(config.default_admin_role(), gov)?;

        Ok(())
    }

    pub fn set_chain_config(
        ctx: Context<SetChainConfig>,
        chain_id: u64,
        block_chain: String,
        router_contract: String,
        extra: String,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.set_chain_config(chain_id, block_chain, router_contract, extra)
    }

    pub fn set_token_config(
        ctx: Context<SetTokenConfig>,
        token_id: String,
        chain_id: u64,
        token_addr: String,
        decimals: u8,
        version: u64,
        router_contract: String,
        underlying: String,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.set_token_config(token_id, chain_id, token_addr, decimals, version, router_contract, underlying)
    }

    pub fn set_swap_config(
        ctx: Context<SetSwapConfig>,
        token_id: String,
        dst_chain_id: u64,
        max_swap: u64,
        min_swap: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.set_swap_config(token_id, dst_chain_id, max_swap, min_swap)
    }

    pub fn set_swap_configs(
        ctx: Context<SetSwapConfigs>,
        token_id: String,
        configs: Vec<SwapConfig>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.set_swap_configs(token_id, configs)
    }

    pub fn set_mpc_pubkey(
        ctx: Context<SetMPCPubkey>,
        addr: String,
        pubkey: String,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.set_mpc_pubkey(addr, pubkey)
    }

    // Add other instruction handlers here...
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + TheiaRouterConfig::MAX_SIZE)]
    pub config: Account<'info, TheiaRouterConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetChainConfig<'info> {
    #[account(mut, has_one = gov)]
    pub config: Account<'info, TheiaRouterConfig>,
    pub gov: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetTokenConfig<'info> {
    #[account(mut, has_one = gov)]
    pub config: Account<'info, TheiaRouterConfig>,
    pub gov: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetSwapConfig<'info> {
    #[account(mut, has_one = gov)]
    pub config: Account<'info, TheiaRouterConfig>,
    pub gov: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetSwapConfigs<'info> {
    #[account(mut, has_one = gov)]
    pub config: Account<'info, TheiaRouterConfig>,
    pub gov: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetMPCPubkey<'info> {
    #[account(mut, has_one = gov)]
    pub config: Account<'info, TheiaRouterConfig>,
    pub gov: Signer<'info>,
}

#[account]
pub struct TheiaRouterConfig {
    pub gov: Pubkey,
    pub c3caller_proxy: Pubkey,
    pub dapp_id: u64,
    pub tx_senders: Vec<Pubkey>,
    pub config_version: u64,
    pub config_role: [u8; 32],
    pub all_chain_ids: Vec<u64>,
    pub all_chain_ids_map: HashMap<u64, bool>,
    pub chain_config: HashMap<u64, ChainConfig>,
    pub all_token_ids: Vec<String>,
    pub all_token_ids_map: HashMap<String, bool>,
    pub all_multichain_tokens: HashMap<String, Vec<MultichainToken>>,
    pub all_swap_configs: HashMap<String, Vec<SwapConfig>>,
    pub token_config: HashMap<String, HashMap<u64, TokenConfig>>,
    pub all_multichain_tokens_map: HashMap<String, HashMap<u64, String>>,
    pub swap_config: HashMap<String, HashMap<u64, HashMap<u64, SwapConfig>>>,
    pub token_id_map: HashMap<u64, HashMap<String, String>>,
    pub mpc_pubkey: HashMap<String, String>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub block_chain: String,
    pub router_contract: String,
    pub extra: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct TokenConfig {
    pub chain_id: u64,
    pub decimals: u8,
    pub contract_address: String,
    pub contract_version: u64,
    pub router_contract: String,
    pub extra: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct MultichainToken {
    pub chain_id: u64,
    pub token_address: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct SwapConfig {
    pub from_chain_id: u64,
    pub to_chain_id: u64,
    pub max_swap: u64,
    pub min_swap: u64,
}

impl TheiaRouterConfig {
    pub const MAX_SIZE: usize = 10000; // Adjust this value based on your needs

    pub fn grant_role(&mut self, role: [u8; 32], account: Pubkey) -> Result<()> {
        // Implement role-based access control logic here
        Ok(())
    }

    pub fn default_admin_role(&self) -> [u8; 32] {
        [0; 32]
    }

    pub fn set_chain_config(
        &mut self,
        chain_id: u64,
        block_chain: String,
        router_contract: String,
        extra: String,
    ) -> Result<()> {
        require!(chain_id > 0, TheiaRouterConfigError::InvalidChainId);

        let config = ChainConfig {
            chain_id,
            block_chain,
            router_contract,
            extra,
        };

        self.chain_config.insert(chain_id, config.clone());

        if !self.all_chain_ids_map.contains_key(&chain_id) {
            self.all_chain_ids.push(chain_id);
            self.all_chain_ids_map.insert(chain_id, true);
        }

        emit!(LogSetChainConfig {
            chain_id,
            block_chain: config.block_chain,
            router_contract: config.router_contract,
            extra: config.extra,
        });

        Ok(())
    }

    pub fn set_token_config(
        &mut self,
        token_id: String,
        chain_id: u64,
        token_addr: String,
        decimals: u8,
        version: u64,
        router_contract: String,
        underlying: String,
    ) -> Result<()> {
        require!(chain_id > 0 && !token_id.is_empty(), TheiaRouterConfigError::InvalidInput);

        let config = TokenConfig {
            chain_id,
            decimals,
            contract_address: token_addr.clone(),
            contract_version: version,
            router_contract,
            extra: underlying,
        };

        self.token_config
            .entry(token_id.clone())
            .or_default()
            .insert(chain_id, config.clone());

        if !self.all_token_ids_map.contains_key(&token_id) {
            self.all_token_ids.push(token_id.clone());
            self.all_token_ids_map.insert(token_id.clone(), true);
        }

        self.set_multichain_token(token_id.clone(), chain_id, token_addr.clone())?;

        emit!(LogSetTokenConfig {
            chain_id,
            decimals,
            contract_version: version,
            token_id,
            contract_address: token_addr,
            router_contract: config.router_contract,
            extra: config.extra,
        });

        Ok(())
    }

    pub fn set_swap_config(
        &mut self,
        token_id: String,
        dst_chain_id: u64,
        max_swap: u64,
        min_swap: u64,
    ) -> Result<()> {
        require!(!token_id.is_empty(), TheiaRouterConfigError::InvalidInput);

        let src_chain_id = Clock::get()?.slot;
        let config = SwapConfig {
            from_chain_id: src_chain_id,
            to_chain_id: dst_chain_id,
            max_swap,
            min_swap,
        };

        self.swap_config
            .entry(token_id.clone())
            .or_default()
            .entry(src_chain_id)
            .or_default()
            .insert(dst_chain_id, config.clone());

        let configs = self.all_swap_configs.entry(token_id.clone()).or_default();
        if let Some(index) = configs.iter().position(|c| c.from_chain_id == src_chain_id && c.to_chain_id == dst_chain_id) {
            configs[index] = config;
        } else {
            configs.push(config);
        }

        Ok(())
    }

    pub fn set_swap_configs(&mut self, token_id: String, configs: Vec<SwapConfig>) -> Result<()> {
        for config in configs {
            self.set_swap_config(token_id.clone(), config.to_chain_id, config.max_swap, config.min_swap)?;
        }
        Ok(())
    }

    pub fn set_mpc_pubkey(&mut self, addr: String, pubkey: String) -> Result<()> {
        self.mpc_pubkey.insert(addr, pubkey);
        Ok(())
    }

    fn set_multichain_token(&mut self, token_id: String, chain_id: u64, token: String) -> Result<()> {
        require!(chain_id > 0 && !token_id.is_empty(), TheiaRouterConfigError::InvalidInput);

        self.token_id_map
            .entry(chain_id)
            .or_default()
            .insert(token.clone(), token_id.clone());

        self.all_multichain_tokens_map
            .entry(token_id.clone())
            .or_default()
            .insert(chain_id, token.clone());

        let tokens = self.all_multichain_tokens.entry(token_id.clone()).or_default();
        if let Some(index) = tokens.iter().position(|t| t.chain_id == chain_id) {
            let old_token = tokens[index].token_address.clone();
            if token != old_token {
                tokens[index].token_address = token.clone();
                self.token_id_map
                    .entry(chain_id)
                    .or_default()
                    .remove(&old_token);
            }
        } else {
            tokens.push(MultichainToken {
                chain_id,
                token_address: token,
            });
        }

        Ok(())
    }

    // Implement getter functions here...
}

#[error_code]
pub enum TheiaRouterConfigError {
    #[msg("Invalid chain ID")]
    InvalidChainId,
    #[msg("Invalid input")]
    InvalidInput,
}

#[event]
pub struct LogSetChainConfig {
    pub chain_id: u64,
    pub block_chain: String,
    pub router_contract: String,
    pub extra: String,
}

#[event]
pub struct LogSetTokenConfig {
    pub chain_id: u64,
    pub decimals: u8,
    pub contract_version: u64,
    pub token_id: String,
    pub contract_address: String,
    pub router_contract: String,
    pub extra: String,
}
