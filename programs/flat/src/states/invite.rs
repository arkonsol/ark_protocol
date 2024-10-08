use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct DAOInvite {
    pub dao: Pubkey,
    pub creator: Pubkey,
    pub created_at: i64,
    pub expires_at: i64,
    pub is_used: bool,
    pub used_by: Option<Pubkey>,
}

impl DAOInvite {
    const OPTION_LENGTH: usize = 1;
    
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2  // dao, creator
        + TIMESTAMP_LENGTH * 2   // created_at, expires_at
        + 1                      // is_used
        + Self::OPTION_LENGTH + PUBLIC_KEY_LENGTH;  // used_by
}