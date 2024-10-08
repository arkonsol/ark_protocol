use anchor_lang::prelude::*;
use crate::states::{GovernancePool, Assembly};
use crate::error::GovernanceError;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct CreateAssembly<'info> {
    #[account(
        mut,
        seeds = [b"governance_pool", admin.key().as_ref()],
        bump =  governance_pool.bump,
    )]
    pub governance_pool: Account<'info, GovernancePool>,
    
    #[account(
        init,
        payer = admin,
        space = Assembly::LEN,
        seeds = [b"assembly", governance_pool.key().as_ref(), &governance_pool.assemblies.len().to_le_bytes()],
        bump
    )]
    pub assembly: Account<'info, Assembly>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

pub fn handler(ctx: Context<CreateAssembly>, name: String, description: String) -> Result<()> {
    require!(name.len() <= MAX_NAME_LENGTH, GovernanceError::InvalidAssembly);
    require!(description.len() <= MAX_DESCRIPTION_LENGTH, GovernanceError::InvalidAssembly);

    let governance_pool = &mut ctx.accounts.governance_pool;
    let assembly = &mut ctx.accounts.assembly;

    governance_pool.add_assembly(assembly.key())?;

    assembly.governance_pool = governance_pool.key();
    assembly.name = name;
    assembly.description = description;
    assembly.members = Vec::new();
    assembly.policy_areas = Vec::new();
    assembly.total_proposals = 0;
    assembly.total_votes = 0;
    assembly.bump = ctx.bumps.assembly;

    Ok(())
}