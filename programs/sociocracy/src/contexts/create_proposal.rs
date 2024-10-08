use anchor_lang::prelude::*;
use crate::states::{circle::Circle, proposal::{Proposal, ProposalStatus}};
use crate::errors::GovernanceError;

#[derive(Accounts)]
#[instruction(description: String)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub circle: Account<'info, Circle>,
    #[account(
        init,
        payer = proposer,
        space = Proposal::SPACE,
        seeds = [b"proposal", circle.key().as_ref(), description.as_bytes()],
        bump
    )]
    pub proposal: Box<Account<'info, Proposal>>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_proposal(ctx: Context<CreateProposal>, description: String) -> Result<()> {
    let circle = &mut ctx.accounts.circle;
    let proposal = &mut ctx.accounts.proposal;
    let proposer = &ctx.accounts.proposer;

    if !circle.members.contains(&proposer.key()) {
        return Err(GovernanceError::NotCircleMember.into());
    }

    proposal.circle = circle.key();
    proposal.proposer = proposer.key();
    proposal.description = description;
    proposal.votes = vec![];
    proposal.status = ProposalStatus::Active;
    proposal.bump = ctx.bumps.proposal;

    circle.proposals.push(proposal.key());

    Ok(())
}