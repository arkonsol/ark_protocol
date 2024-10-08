use anchor_lang::prelude::*;

#[account]
pub struct Vote {
    pub governance_pool: Pubkey,
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub decision: VoteDecision,
    pub voting_power: u64,
    pub timestamp: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum VoteDecision {
    Approve,
    Reject,
    Abstain,
}


impl Vote {
    pub const LEN: usize = 8 + // discriminator
        32 + // governance_pool
        32 + // proposal
        32 + // voter
        1 + // approve
        8; // timestamp

}