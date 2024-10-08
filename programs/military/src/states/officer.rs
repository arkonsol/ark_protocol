use anchor_lang::prelude::*;

#[account]
pub struct Officer {
    pub authority: Pubkey,
    pub rank: u8,
    pub appointed_at: i64,
    pub bump: u8
}

impl Officer {
    pub const SPACE: usize = 8 + //discriminator
    32 + //authority
    1 + // rank
    8 + //appointed_at
    1; // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Rank {
    General = 0,
    Colonel = 1,
    Major = 2,
    Captain = 3,
}