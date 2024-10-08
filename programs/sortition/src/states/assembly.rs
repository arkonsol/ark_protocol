use anchor_lang::prelude::*;

#[account]
pub struct Assembly {
    pub governance_pool: Pubkey,
    pub members: Vec<Pubkey>,
    pub term_start: i64,
    pub term_end: i64,
    pub proposals: Vec<Pubkey>,
    pub bump: u8,
}

impl Assembly {
    pub const MAX_MEMBERS: usize = 100;
    pub const MAX_PROPOSALS: usize = 1000;

    pub const SPACE: usize = 8 + // discriminator
        32 + // governance_pool
        4 + (32 * Self::MAX_MEMBERS) + // members
        8 + // term_start
        8 + // term_end
        4 + (32 * Self::MAX_PROPOSALS) + // proposals
        1;
}