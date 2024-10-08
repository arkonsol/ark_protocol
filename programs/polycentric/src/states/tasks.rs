use anchor_lang::prelude::*;

#[account]
pub struct Task {
    pub id: u64,
    pub governance_pool: Pubkey,
    pub description: String,
    pub reward: u64,
    pub completed_by: Option<Pubkey>,
    pub bump: u8,
}