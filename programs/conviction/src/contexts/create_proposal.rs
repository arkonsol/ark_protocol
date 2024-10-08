use anchor_lang::prelude::*;
use crate::states::{governance::Governance, proposal::{Proposal, ProposalType, ProposalStatus}};

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub governance: Box<Account<'info, Governance>>,
    #[account(
        init, 
        payer = creator, 
        space = Proposal::SPACE,
        seeds = [b"proposal", governance.key().as_ref(), &governance.total_proposals.to_le_bytes()],
        bump
    )]
    pub proposal: Box<Account<'info, Proposal>>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_proposal(
    ctx: Context<CreateProposal>,
    description: String,
    voting_period: i64,
    execution_delay: i64,
    proposal_type: ProposalType,
) -> Result<()> {

    // pub id: u64,
    // pub description: String,
    // pub creator: Pubkey,
    // pub start_time: i64,
    // pub end_time: i64,
    // pub execution_time: i64,
    // pub for_votes: u64,
    // pub against_votes: u64,
    // pub status: ProposalStatus,
    // pub param_name: Option<String>,
    // pub param_value: Option<u64>,
    // pub transfer_amount: Option<u64>,
    // pub bump: u8,

    let governance = &mut ctx.accounts.governance;
    let proposal = &mut ctx.accounts.proposal;

    proposal.id = governance.total_proposals + 1;
    proposal.description = description;
    proposal.creator = ctx.accounts.creator.key();
    proposal.start_time = Clock::get()?.unix_timestamp;
    proposal.end_time = proposal.start_time + voting_period;
    proposal.execution_time = proposal.end_time + execution_delay;
    proposal.for_votes = 0;
    proposal.against_votes = 0;
    proposal.status = ProposalStatus::Active;
    proposal.param_name = None;
    proposal.param_value = None;
    proposal.transfer_amount = None;
    proposal.proposal_type = proposal_type; 

    governance.total_proposals += 1;
    governance.total_active_proposals += 1;

    Ok(())
}

