use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::GovernanceError;

#[account]
pub struct Assembly {
    pub governance_pool: Pubkey,
    pub name: String,
    pub description: String,
    pub members: Vec<Pubkey>,
    pub policy_areas: Vec<Pubkey>,
    pub total_proposals: u64,
    pub total_votes: u64,
    pub bump: u8,

}

impl Assembly {
    pub const LEN: usize = 8 + // discriminator
        32 + // governance_pool
        4 + MAX_NAME_LENGTH + // name
        4 + MAX_DESCRIPTION_LENGTH + // description
        4 + (32 * MAX_ASSEMBLY_MEMBERS) + // members
        4 + (32 * MAX_POLICY_AREAS_PER_ASSEMBLY) + // policy_areas
        8 + // total_proposals
        8 + // total_votes
        1;
        
    pub fn add_member(&mut self, member: Pubkey) -> Result<()> {
        require!(self.members.len() < MAX_ASSEMBLY_MEMBERS, GovernanceError::MaxAssemblyMembersReached);
        self.members.push(member);
        Ok(())
    }

    pub fn add_policy_area(&mut self, policy_area: Pubkey) -> Result<()> {
        require!(self.policy_areas.len() < MAX_POLICY_AREAS_PER_ASSEMBLY, GovernanceError::MaxPolicyAreasPerAssemblyReached);
        self.policy_areas.push(policy_area);
        Ok(())
    }

    pub fn increment_proposals(&mut self) {
        self.total_proposals += 1;
    }

    pub fn increment_votes(&mut self) {
        self.total_votes += 1;
    }
}