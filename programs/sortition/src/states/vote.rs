use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]

pub struct Vote {
    pub voter: Pubkey,
    pub vote: VoteDecision,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum VoteDecision {
    Approve,
    Reject,
    Abstain,
}