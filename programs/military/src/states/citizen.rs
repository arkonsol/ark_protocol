use anchor_lang::prelude::*;

#[derive(Copy)]
#[account]
pub struct Citizen {
    pub authority: Pubkey,
    pub loyalty_score: u8,
    pub resources: u64,
    pub is_dissident: bool,
    pub is_imprisoned: bool,
    pub imprisonment_end: Option<i64>,
    pub joined_at: i64,
    pub bump: u8,
}

impl Citizen {
    pub const LEN: usize = 8 + // account discriminator
        32 + // authority
        1 + // loyalty_score
        8 + // resources
        1 + // is_dissident
        1 + // is_imprisoned
        8 + // imprisonment_end (Option<i64>)
        8 + // joined_at
        1; // bump
}