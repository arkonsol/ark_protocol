use anchor_lang::prelude::*;

#[account]
pub struct Vote {
    pub governance: Pubkey,
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub amount: u64,
    pub conviction: u64,
    pub last_update: i64,
    pub vote_for: Pubkey,
    pub vote: Option<bool>,
    pub power: u64,
    pub bump: u8,
}

#[account]
pub struct StakeAccount {
    pub user: Pubkey,
    pub amount: u64,
    pub nft_amount: u64,
    pub lock_end: i64,
    pub conviction_multiplier: u8,
    pub bump: u8,
}


impl Vote {
    pub const SPACE: usize = 8 + // discriminator
        32 + // governance
        32 + // proposal
        32 + // voter
        8 + // amount
        8 + // conviction
        8 + // last_update
        32 + // vote_for
        1 + 1 + // vote
        8 + // power
        1;
}

impl StakeAccount {
    pub const SPACE: usize = 8 + 32 + 8 + 8 + 8 + 1 + 1; // Adjust based on data structure size.
}