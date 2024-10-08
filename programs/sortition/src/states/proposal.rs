use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub assembly: Pubkey,
    pub proposer: Pubkey,
    pub name: String,
    pub description: String,
    pub votes: Vec<(Pubkey, bool)>, // (member, approval)
    pub start_time: i64,
    pub end_time: i64, 
    pub status: ProposalStatus,
    pub created_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
}

impl Proposal {
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_DESCRIPTION_LENGTH: usize = 200;
    pub const MAX_VOTES: usize = 100;

    pub const SPACE: usize = 8 + // discriminator
        32 + // assembly
        32 + // proposer
        4 + Self::MAX_NAME_LENGTH + // description
        4 + Self::MAX_DESCRIPTION_LENGTH + // description
        4 + (Self::MAX_VOTES * (32 + 1)) + // votes
        8 + //start_time
        8 + //end_time
        1 + // status
        8 +
        1;
}