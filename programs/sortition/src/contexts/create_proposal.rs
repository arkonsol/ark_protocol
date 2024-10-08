use anchor_lang::prelude::*;
use crate::states::{assembly::Assembly, proposal::{Proposal, ProposalStatus}};
use crate::error::GovernanceError;

#[derive(Accounts)]
#[instruction(description: String)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub assembly: Account<'info, Assembly>,
    #[account(
        init,
        payer = proposer,
        space = Proposal::SPACE,
        seeds = [b"proposal", assembly.key().as_ref(), description.as_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

pub fn create_proposal(ctx: Context<CreateProposal>, name: String, description: String, end_time: i64) -> Result<()> {
    let assembly = &mut ctx.accounts.assembly;
    let proposal = &mut ctx.accounts.proposal;
    let proposer = &ctx.accounts.proposer;

    require!(assembly.members.contains(&proposer.key()), GovernanceError::NotAssemblyMember);
    require!(Clock::get()?.unix_timestamp < assembly.term_end, GovernanceError::AssemblyTermEnded);
    require!(description.len() <= Proposal::MAX_DESCRIPTION_LENGTH, GovernanceError::InvalidInput);

    proposal.assembly = assembly.key();
    proposal.name = name;
    proposal.proposer = proposer.key();
    proposal.description = description;
    proposal.votes = vec![];
    proposal.start_time = Clock::get()?.unix_timestamp;
    proposal.end_time = end_time;
    proposal.status = ProposalStatus::Active;
    proposal.created_at = Clock::get()?.unix_timestamp;

    assembly.proposals.push(proposal.key());

    Ok(())
}