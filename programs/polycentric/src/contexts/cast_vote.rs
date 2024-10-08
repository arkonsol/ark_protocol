use anchor_lang::prelude::*;
use crate::states::{GovernancePool, PolicyArea, Assembly, Citizen, Proposal, Vote, ProposalStatus, VoteDecision};
use crate::error::GovernanceError;

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,
    
    #[account(mut)]
    pub policy_area: Account<'info, PolicyArea>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = voter,
        space = Vote::LEN,
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    #[account(
        mut,
        constraint = assembly.governance_pool == governance_pool.key() 
        && assembly.policy_areas.contains(&policy_area.key())
    )]
    pub assembly: Account<'info, Assembly>,
    
    #[account(
        mut,
        seeds = [b"citizen", governance_pool.key().as_ref(), voter.key().as_ref()],
        bump = citizen.bump
    )]
    pub citizen: Account<'info, Citizen>,
    
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<CastVote>, decision: VoteDecision, voting_power_to_use: u64) -> Result<()> {
    let governance_pool = &mut ctx.accounts.governance_pool;
    let policy_area = &mut ctx.accounts.policy_area;
    let proposal = &mut ctx.accounts.proposal;
    let vote = &mut ctx.accounts.vote;
    let voter = &ctx.accounts.voter;
    let assembly = &mut ctx.accounts.assembly;
    let citizen = &mut ctx.accounts.citizen;
    let clock = &ctx.accounts.clock;

    // Ensure the voter is a member of the assembly
    require!(assembly.members.contains(&voter.key()), GovernanceError::NotAssemblyMember);

    // Ensure the proposal is still active
    require!(proposal.status == ProposalStatus::Active, GovernanceError::ProposalNotActive);

    require!(citizen.voting_power >= voting_power_to_use, GovernanceError::InsufficientVotingPower);

    // Ensure the voting period hasn't ended
    let current_time = clock.unix_timestamp;
    require!(current_time <= proposal.end_time, GovernanceError::ProposalVotingEnded);

    // Ensure the citizen has voting power
    require!(citizen.voting_power > 0, GovernanceError::InsufficientVotingPower);

    // Cast the vote with voting power
    match decision {
        VoteDecision::Approve => {
            proposal.yes_votes += voting_power_to_use;
        },
        VoteDecision::Reject => {
            proposal.no_votes += voting_power_to_use;
        },
        VoteDecision::Abstain => {
            proposal.abstain_votes += voting_power_to_use;
        },
    }

    citizen.voting_power -= voting_power_to_use;

    // Create the vote record
    vote.governance_pool = governance_pool.key();
    vote.proposal = proposal.key();
    vote.voter = voter.key();
    vote.decision = decision;
    vote.voting_power = voting_power_to_use;
    vote.timestamp = current_time;
    vote.bump = ctx.bumps.vote;

    // Update counters
    governance_pool.total_votes += 1;
    policy_area.total_votes += 1;
    assembly.total_votes += 1;

    // Check if the proposal has reached a conclusion
    if proposal.has_reached_conclusion() {
        proposal.finalize_proposal()?;
    }

    Ok(())
}