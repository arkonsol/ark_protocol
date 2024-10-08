use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::GovernanceError;

#[account]
pub struct Proposal {
    pub governance_pool: Pubkey,
    pub policy_area: Pubkey,
    pub creator: Pubkey,
    pub title: String,
    pub description: String,
    pub status: ProposalStatus,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub abstain_votes: u64,
    pub quorum_threshold: u64,
    pub approval_threshold: u8, 
    pub start_time: i64,
    pub end_time: i64,
    pub bump: u8,

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
    Executed,
}

impl Proposal {
    pub const LEN: usize = 8 + // discriminator
        32 + // governance_pool
        32 + // policy_area
        32 + // creator
        4 + MAX_TITLE_LENGTH + // title
        4 + MAX_DESCRIPTION_LENGTH + // description
        1 + // status (enum)
        8 + // yes_votes
        8 + // no_votes
        8 + // start_time
        8; // end_time

    pub fn has_reached_conclusion(&self) -> bool {
        let total_votes = self.yes_votes + self.no_votes + self.abstain_votes;
        
        // Check if quorum is reached
        if total_votes < self.quorum_threshold {
            return false;
        }

        // Calculate if approval threshold is met using integer arithmetic
        // Assuming approval_threshold is stored as a percentage (e.g., 51 for 51%)
        let approval_threshold = self.approval_threshold;
        let required_yes_votes = (total_votes * approval_threshold as u64) / 100;

        self.yes_votes >= required_yes_votes
    }

    pub fn finalize_proposal(&mut self) -> Result<()> {
        if self.yes_votes > self.no_votes {
            self.status = ProposalStatus::Approved;
        } else {
            self.status = ProposalStatus::Rejected;
        }
        Ok(())
    }

    pub fn execute(&mut self) -> Result<()> {
        require!(self.status == ProposalStatus::Approved, GovernanceError::ProposalNotApproved);

        self.status = ProposalStatus::Executed;

        Ok(())
    }
}