use anchor_lang::prelude::*;

#[account]
pub struct GovernanceInvite {
    pub governance_pool: Pubkey,
    pub creator: Pubkey,
    pub created_at: i64,
    pub expires_at: i64,
    pub is_used: bool,
    pub used_by: Option<Pubkey>,
    pub bump: u8,
}

impl GovernanceInvite {
    pub const SPACE: usize = 8 + // discriminator
        32 + // governance_pool
        32 + // creator
        8 +  // created_at
        8 +  // expires_at
        1 +  // is_used
        33 + // used_by (1 byte for Option + 32 bytes for Pubkey)
        1;  // bump
}