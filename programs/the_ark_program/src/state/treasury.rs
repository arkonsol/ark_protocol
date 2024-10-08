use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

#[account]
pub struct Treasury {
    pub name: String,
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub tokens: Vec<TokenAccount>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenAccount {
    pub mint: Pubkey,
    pub account: Pubkey,
}

impl Treasury {
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_TOKEN_ACCOUNTS: usize = 5;

    pub const LEN: usize = 8 + // discriminator
        4 + Self::MAX_NAME_LENGTH + // name
        32 + // owner
        32 + // authority
        4 + (64 * Self::MAX_TOKEN_ACCOUNTS); // tokens (Vec<TokenAccount>)

    pub fn add_token_account(&mut self, mint: Pubkey, account: Pubkey) -> Result<()> {
        require!(self.tokens.len() < Self::MAX_TOKEN_ACCOUNTS, ErrorCode::MaxTokenAccountsReached);
        self.tokens.push(TokenAccount { mint, account });
        Ok(())
    }
}