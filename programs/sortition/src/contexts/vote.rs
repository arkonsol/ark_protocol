use anchor_lang::prelude::*;
use crate::states::{assembly::Assembly, proposal::{Proposal, ProposalStatus}};
use crate::error::GovernanceError;

#[derive(Accounts)]
pub struct VoteOnProposal<'info> {
    #[account(mut)]
    pub assembly: Account<'info, Assembly>,
    #[account(
        mut,
        constraint = proposal.assembly == assembly.key() @ GovernanceError::InvalidProposal
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub member: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn vote_on_proposal(ctx: Context<VoteOnProposal>, approve: bool) -> Result<()> {
    let assembly = &ctx.accounts.assembly;
    let proposal = &mut ctx.accounts.proposal;
    let member = &ctx.accounts.member;
    let current_time = ctx.accounts.clock.unix_timestamp;

    require!(assembly.members.contains(&member.key()), GovernanceError::NotAssemblyMember);
    require!(proposal.status == ProposalStatus::Active, GovernanceError::ProposalNotActive);
    require!(current_time < assembly.term_end, GovernanceError::AssemblyTermEnded);
    require!(current_time >= proposal.start_time, GovernanceError::VotingNotStarted);
    require!(current_time <= proposal.end_time, GovernanceError::VotingEnded);

    // Remove existing vote if any
    proposal.votes.retain(|(voter, _)| voter != &member.key());
    
    // Add new vote
    proposal.votes.push((member.key(), approve));

    // Check if all members have voted or if voting period has ended
    if proposal.votes.len() == assembly.members.len() || current_time >= proposal.end_time {
        let approvals = proposal.votes.iter().filter(|(_, approve)| *approve).count();
        let threshold = assembly.members.len() * 2 / 3; // 2/3 majority

        proposal.status = if approvals > threshold {
            ProposalStatus::Passed
        } else {
            ProposalStatus::Rejected
        };
    }

    emit!(VoteEvent {
        proposal: proposal.key(),
        voter: member.key(),
        approve,
        timestamp: current_time,
    });

    Ok(())
}

#[event]
pub struct VoteEvent {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub approve: bool,
    pub timestamp: i64,
}