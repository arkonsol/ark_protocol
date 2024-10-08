use anchor_lang::prelude::*;
use crate::states::{governance::GovernancePool, assembly::Assembly, pagination::PaginationState};
use crate::error::GovernanceError;
use anchor_lang::solana_program::hash::hash;

#[derive(Accounts)]
pub struct SelectAssembly<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,
    #[account(
        init,
        payer = admin,
        space = Assembly::SPACE,
        seeds = [b"assembly", governance_pool.key().as_ref(), &governance_pool.total_citizens.to_le_bytes()],
        bump
    )]
    pub assembly: Account<'info, Assembly>,
    #[account(
        init,
        payer = admin,
        space = PaginationState::SPACE,
        seeds = [b"pagination", governance_pool.key().as_ref(), &governance_pool.total_citizens.to_le_bytes()],
        bump
    )]
    pub pagination_state: Account<'info, PaginationState>,
    #[account(mut, constraint = admin.key() == governance_pool.admin)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn select_assembly(ctx: Context<SelectAssembly>, term_length: i64) -> Result<()> {
    let governance_pool = &mut ctx.accounts.governance_pool;
    let assembly = &mut ctx.accounts.assembly;
    let pagination_state = &mut ctx.accounts.pagination_state;

    require!(!governance_pool.selection_in_progress, GovernanceError::RandomSelectionInProgress);
    require!(term_length > 0 && term_length <= 31536000, GovernanceError::InvalidTermLength); // Max 1 year

    if governance_pool.current_assembly != Pubkey::default() && 
       governance_pool.assembly_term > Clock::get()?.unix_timestamp {
        return Err(GovernanceError::AssemblyAlreadyActive.into());
    }

    if governance_pool.total_citizens < governance_pool.assembly_size as u32 {
        return Err(GovernanceError::NotEnoughCitizens.into());
    }

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    // Start the random selection process
    governance_pool.selection_in_progress = true;
    governance_pool.last_random_seed = hash(&clock.slot.to_le_bytes()).to_bytes();

    assembly.governance_pool = governance_pool.key();
    assembly.term_start = current_timestamp;
    assembly.term_end = current_timestamp + term_length;
    assembly.members = vec![];
    assembly.proposals = vec![];

    governance_pool.current_assembly = assembly.key();
    governance_pool.assembly_term = assembly.term_end;

    // Initialize pagination state
    pagination_state.governance_pool = governance_pool.key();
    pagination_state.current_index = 0;
    pagination_state.current_citizen_in_index = 0;
    pagination_state.selected_citizens = vec![];
    pagination_state.demographic_counts = governance_pool.demographic_quotas.clone();
    pagination_state.demographic_counts.reset();

    Ok(())
}