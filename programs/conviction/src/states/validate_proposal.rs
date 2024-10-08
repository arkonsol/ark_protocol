use anchor_lang::prelude::*;
use crate::states::{proposal::{Proposal, ProposalStatus}, governance::Governance};

pub fn check_proposal_status(proposal: &mut Proposal, governance: &Governance) -> Result<()> {

    let total_votes = proposal.for_votes + proposal.against_votes;

    if total_votes == 0 {
        return Ok(());
    }

    let approval_ratio = (proposal.for_votes as f64) / (total_votes as f64);
    let approval_threshold = (governance.approval_threshold as f64) / 100.0;

    if approval_ratio > approval_threshold {
        proposal.status = ProposalStatus::Passed;
    }

    Ok(())
}