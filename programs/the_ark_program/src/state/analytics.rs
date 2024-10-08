// To count the number of instances that have been initialized from all governments and the number of 
// polls, approved proposals and disapproved proposal

use anchor_lang::prelude::*;
use crate::constants::*;


#[account]
pub struct ArkAnalytics {
    pub total_governments: u64,
    pub governments: Vec<Pubkey>,
    pub initialized_at: i64,
    pub bump: u8,
}

impl ArkAnalytics {
    const MAX_NO_OF_GOVERNMENTS: usize = 50;

    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + 8
        + (Self::MAX_NO_OF_GOVERNMENTS * 32)
        + TIMESTAMP_LENGTH // initialized_at
        + BUMP_LENGTH;
    }