use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub circle: Pubkey,
    pub proposer: Pubkey,
    pub description: String,
    pub votes: Vec<(Pubkey, bool)>, 
    pub status: ProposalStatus,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
}

impl Proposal {
    pub const MAX_DESCRIPTION_LENGTH: usize = 200;
    pub const MAX_VOTES: usize = 20;

    pub const SPACE: usize =
        8 + // discriminator
        32 + // circle
        32 + // proposer
        4 + Self::MAX_DESCRIPTION_LENGTH + // description
        4 + (Self::MAX_VOTES * (32 + 1)) + // votes
        1 + // status
        1;
}