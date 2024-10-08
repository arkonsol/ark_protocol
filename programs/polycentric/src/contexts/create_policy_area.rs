use anchor_lang::prelude::*;
use crate::states::{GovernancePool, PolicyArea};
use crate::error::GovernanceError;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct CreatePolicyArea<'info> {
    #[account(
        mut,
        seeds = [b"governance_pool", admin.key().as_ref()],
        bump = governance_pool.bump,
    )]
    pub governance_pool: Account<'info, GovernancePool>,
    
    #[account(
        init,
        payer = admin,
        space = PolicyArea::LEN,
        seeds = [b"policy_area", governance_pool.key().as_ref(), &governance_pool.policy_areas.len().to_le_bytes()],
        bump
    )]
    pub policy_area: Account<'info, PolicyArea>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

pub fn handler(ctx: Context<CreatePolicyArea>, name: String, description: String) -> Result<()> {
    require!(name.len() <= MAX_NAME_LENGTH, GovernanceError::InvalidPolicyArea);
    require!(description.len() <= MAX_DESCRIPTION_LENGTH, GovernanceError::InvalidPolicyArea);
    

    let governance_pool = &mut ctx.accounts.governance_pool;
    let policy_area = &mut ctx.accounts.policy_area;

    require!(
        governance_pool.policy_areas.len() < MAX_POLICY_AREAS,
        GovernanceError::MaxPolicyAreasReached
    );

    governance_pool.add_policy_area(policy_area.key())?;

    policy_area.governance_pool = governance_pool.key();
    policy_area.name = name;
    policy_area.description = description;
    policy_area.assemblies = Vec::new();
    policy_area.proposals = Vec::new();
    policy_area.total_votes = 0;
    policy_area.bump = ctx.bumps.policy_area;

    Ok(())
}