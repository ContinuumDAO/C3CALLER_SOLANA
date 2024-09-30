use anchor_lang::error_code;

#[error_code]
pub enum TheiaError{
 #[msg("uuid generation failed")]
 GenUUIDCPIFailed,
 #[msg("cpi call to c3call failed")]
 CcallCpiFailed

}