use anchor_lang::error_code;

#[error_code]
pub enum C3CallerErros{
    #[msg("C3Caller: empty dappI")]
    DappIdisEmpty,
    #[msg("C3Caller: empty _to")]
    ToisEmpty,
    #[msg("C3Caller: empty toChainID")]
    ToChainIdisEmpty,
    #[msg("C3Caller: empty calldata")]
    DataisEmpty



}