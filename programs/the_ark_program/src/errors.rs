use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of token accounts reached")]
    MaxTokenAccountsReached,
    #[msg("Name is too long")]
    NameTooLong
}