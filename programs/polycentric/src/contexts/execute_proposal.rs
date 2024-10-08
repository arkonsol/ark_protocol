use anchor_lang::prelude::*;
use crate::states::{GovernancePool, PolicyArea, Assembly, Proposal, ProposalStatus};
use crate::error::GovernanceError;

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(
        mut,
        seeds = [b"governance_pool", admin.key().as_ref()],
        bump =  governance_pool.bump,
        has_one = admin
    )]
    pub governance_pool: Account<'info, GovernancePool>,
    
    #[account(mut,
    seeds = [b"policy_area", governance_pool.key().as_ref(), &governance_pool.policy_areas.len().to_le_bytes()],
    bump = policy_area.bump
    )]
    pub policy_area: Account<'info, PolicyArea>,
    
    #[account(
        mut,
        seeds = [b"proposal", policy_area.key().as_ref(), &policy_area.proposals.len().to_le_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        constraint = assembly.governance_pool == governance_pool.key() 
        && assembly.policy_areas.contains(&policy_area.key())
    )]
    pub assembly: Account<'info, Assembly>,
    
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<ExecuteProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let executor = &ctx.accounts.admin;
    let assembly = &ctx.accounts.assembly;
    let clock = &ctx.accounts.clock;

    // Ensure the executor is a member of the assembly
    require!(assembly.members.contains(&executor.key()), GovernanceError::NotAssemblyMember);

    // Ensure the voting period has ended
    let current_time = clock.unix_timestamp;
    require!(current_time > proposal.end_time, GovernanceError::ProposalVotingEnded);

    // Finalize the proposal if it hasn't been done yet
    if proposal.status == ProposalStatus::Active {
        proposal.finalize_proposal()?;
    }

    // Execute the proposal
    proposal.execute()?;

    // Here, we would typically implement the actual execution logic
    // This could involve calling other instructions, transferring funds, etc.
    // For now, we'll just emit an event

    emit!(ProposalExecuted {
        governance_pool: ctx.accounts.governance_pool.key(),
        policy_area: ctx.accounts.policy_area.key(),
        proposal: proposal.key(),
        executor: executor.key(),
    });

    Ok(())
}

#[event]
pub struct ProposalExecuted {
    pub governance_pool: Pubkey,
    pub policy_area: Pubkey,
    pub proposal: Pubkey,
    pub executor: Pubkey,
}