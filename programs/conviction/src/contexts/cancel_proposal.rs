use anchor_lang::prelude::*;
use crate::states::{proposal::{Proposal, ProposalStatus}, governance::Governance};
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct CancelProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = proposal.creator == authority.key() @ ErrorCode::Unauthorized,
        constraint = proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive
    )]
    pub proposal: Box<Account<'info, Proposal>>,
    #[account(mut)]
    pub governance: Box<Account<'info, Governance>>,
}

pub fn cancel_proposal(ctx: Context<CancelProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let governance = &mut ctx.accounts.governance;

    // Update proposal status
    proposal.status = ProposalStatus::Cancelled;

    // Update governance state 
    governance.total_active_proposals = governance.total_active_proposals.saturating_sub(1);
    governance.total_proposals = governance.total_proposals.saturating_sub(1);


    // Emit an event for the cancellation
    emit!(ProposalCancelled {
        proposal_id: proposal.id,
        canceller: ctx.accounts.authority.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct ProposalCancelled {
    pub proposal_id: u64,
    pub canceller: Pubkey,
    pub timestamp: i64,
}