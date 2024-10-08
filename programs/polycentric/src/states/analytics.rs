use anchor_lang::prelude::*;

#[account]
pub struct Analytics {
    pub total_governance_pools: u64,
    pub total_assemblies: u64,
    pub total_policy_areas: u64,
    pub total_proposals: u64,
    pub total_votes: u64,
    pub last_updated: i64,
    pub counted_pools: Vec<Pubkey>,
    pub bump: u8,
}

impl Analytics {
    pub const MAX_COUNTED_POOLS: usize = 5; 

    pub const LEN: usize = 8 + // discriminator
        8 + // total_governance_pools
        8 + // total_assemblies
        8 + // total_policy_areas
        8 + // total_proposals
        8 + // total_votes
        8 + // last_updated
        4 + (32 * Self::MAX_COUNTED_POOLS) + // counted_pools (Vec<Pubkey>)
        1;

    pub fn increment_governance_pools(&mut self) {
        self.total_governance_pools += 1;
    }

    pub fn increment_assemblies(&mut self) {
        self.total_assemblies += 1;
    }

    pub fn increment_policy_areas(&mut self) {
        self.total_policy_areas += 1;
    }

    pub fn increment_proposals(&mut self) {
        self.total_proposals += 1;
    }

    pub fn increment_votes(&mut self) {
        self.total_votes += 1;
    }

    pub fn update_timestamp(&mut self, timestamp: i64) {
        self.last_updated = timestamp;
    }
}