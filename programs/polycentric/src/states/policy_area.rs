use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::GovernanceError;

#[account]
pub struct PolicyArea {
    pub governance_pool: Pubkey,
    pub name: String,
    pub description: String,
    pub assemblies: Vec<Pubkey>,
    pub proposals: Vec<Pubkey>,
    pub total_votes: u64,
    pub bump: u8,

}

impl PolicyArea {
    pub const LEN: usize = 8 + // discriminator
        32 + // governance_pool
        4 + MAX_NAME_LENGTH + // name
        4 + MAX_DESCRIPTION_LENGTH + // description
        4 + (32 * MAX_ASSEMBLIES_PER_POLICY_AREA) + // assemblies
        4 + (32 * MAX_PROPOSALS_PER_POLICY_AREA) + // proposals
        8; // total_votes


    pub fn add_assembly(&mut self, assembly: Pubkey) -> Result<()> {
        require!(self.assemblies.len() < MAX_ASSEMBLIES_PER_POLICY_AREA, GovernanceError::MaxAssembliesPerPolicyAreaReached);
        self.assemblies.push(assembly);
        Ok(())
    }

    pub fn add_proposal(&mut self, proposal: Pubkey) -> Result<()> {
        require!(self.proposals.len() < MAX_PROPOSALS_PER_POLICY_AREA, GovernanceError::MaxProposalsPerPolicyAreaReached);
        self.proposals.push(proposal);
        Ok(())
    }

    pub fn increment_votes(&mut self) {
        self.total_votes += 1;
    }
}