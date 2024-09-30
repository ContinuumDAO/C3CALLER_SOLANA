use anchor_lang::{error_code};

#[error_code]
pub enum C3CallerErros{
    #[msg("C3Caller: empty dappI")]
    DappIdisEmpty,
    #[msg("C3Caller: empty _to")]
    ToisEmpty,
    #[msg("C3Caller: empty toChainID")]
    ToChainIdisEmpty,
    #[msg("C3Caller: empty calldata")]
    DataisEmpty,
    #[msg("C3Caller: calldata length dismatch")]
    CallDataLengthMismatch,
    #[msg("C3Caller: contracted is paused")]
    ContractIsPaused,
    #[msg("C3Caller: not the owner")]
    NotOwner,
    #[msg("C3Caller:Target program account is different from the target program ID")]
    TargetProgramIdMismatch



}