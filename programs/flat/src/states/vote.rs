use anchor_lang::prelude::*;

use crate::constants::*;
use crate::states::Choice;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Vote {
    pub user: Pubkey,
    pub voting_power: u64,
    pub choice: Choice,
    pub created_at: i64,
}

impl Vote {
    pub const LEN: usize = PUBLIC_KEY_LENGTH 
        + 8 // voting_power 
        + 1 // enum
        + TIMESTAMP_LENGTH;
}

