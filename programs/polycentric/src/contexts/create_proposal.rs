use anchor_lang::prelude::*;
use crate::states::{GovernancePool, PolicyArea, Assembly, Proposal, ProposalStatus};
use crate::error::GovernanceError;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateProposal<'info> {
    #[account(
        mut,
        seeds = [b"governance_pool", admin.key().as_ref()],
        bump =  governance_pool.bump,
    )]
    pub governance_pool: Account<'info, GovernancePool>,
    
    #[account(mut)]
    pub policy_area: Account<'info, PolicyArea>,
    
    #[account(
        init,
        payer = admin,
        space = Proposal::LEN,
        seeds = [b"proposal", policy_area.key().as_ref(), &policy_area.proposals.len().to_le_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"assembly", governance_pool.key().as_ref(), &governance_pool.assemblies.len().to_le_bytes()],
        bump = assembly.bump,
        constraint = assembly.governance_pool == governance_pool.key() 
        && assembly.policy_areas.contains(&policy_area.key())
    )]
    pub assembly: Account<'info, Assembly>,
    
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateProposal>, title: String, description: String) -> Result<()> {
    require!(title.len() <= MAX_TITLE_LENGTH, GovernanceError::InvalidProposal);
    require!(description.len() <= MAX_DESCRIPTION_LENGTH, GovernanceError::InvalidProposal);

    let governance_pool = &mut ctx.accounts.governance_pool;
    let policy_area = &mut ctx.accounts.policy_area;
    let proposal = &mut ctx.accounts.proposal;
    let creator = &ctx.accounts.admin;
    let assembly = &mut ctx.accounts.assembly;
    let clock = &ctx.accounts.clock;

    // Ensure the creator is a member of the assembly
    require!(assembly.members.contains(&creator.key()), GovernanceError::NotAssemblyMember);

    let start_time = clock.unix_timestamp;
    let end_time = start_time + VOTING_PERIOD;

    // pub governance_pool: Pubkey,
    // pub policy_area: Pubkey,
    // pub creator: Pubkey,
    // pub title: String,
    // pub description: String,
    // pub status: ProposalStatus,
    // pub yes_votes: u64,
    // pub no_votes: u64,
    // pub start_time: i64,
    // pub end_time: i64,

        proposal.governance_pool = governance_pool.key();
        proposal.policy_area = policy_area.key();
        proposal.creator = creator.key();
        proposal.title = title;
        proposal.description = description;
        proposal.status = ProposalStatus::Active;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        proposal.start_time = start_time;
        proposal.end_time = end_time;
        proposal.bump = ctx.bumps.proposal;


    policy_area.add_proposal(proposal.key())?;
    governance_pool.increment_proposals();
    assembly.increment_proposals();

    Ok(())
}