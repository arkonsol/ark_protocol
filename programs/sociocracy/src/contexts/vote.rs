use anchor_lang::prelude::*;
use crate::states::{circle::Circle, proposal::{Proposal, ProposalStatus}};
use crate::errors::GovernanceError;

#[derive(Accounts)]
pub struct VoteOnProposal<'info> {
    #[account(mut)]
    pub circle: Box<Account<'info, Circle>>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub member: Signer<'info>,
}

pub fn vote_on_proposal(ctx: Context<VoteOnProposal>, consent: bool) -> Result<()> {
    let circle = &ctx.accounts.circle;
    let proposal = &mut ctx.accounts.proposal;
    let member = &ctx.accounts.member;

    if !circle.members.contains(&member.key()) {
        return Err(GovernanceError::NotCircleMember.into());
    }

    if proposal.status != ProposalStatus::Active {
        return Err(GovernanceError::ProposalNotActive.into());
    }

    // Remove existing vote if any
    proposal.votes.retain(|(voter, _)| voter != &member.key());
    
    // Add new vote
    proposal.votes.push((member.key(), consent));

    // Check if all members have voted and there are no objections
    if proposal.votes.len() == circle.members.len() {
        if proposal.votes.iter().all(|(_, consent)| *consent) {
            proposal.status = ProposalStatus::Passed;
        } else {
            proposal.status = ProposalStatus::Rejected;
        }
    }

    Ok(())
}