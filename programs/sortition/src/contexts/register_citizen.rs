use anchor_lang::prelude::*;
use crate::states::{governance::GovernancePool, citizen::Citizen, citizen_index::CitizenIndex};
use crate::error::GovernanceError;

#[derive(Accounts)]
#[instruction(name: String, region: u8, age_group: u8, other_demographic: u8)]
pub struct RegisterCitizen<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,
    #[account(
        init,
        payer = citizen,
        space = Citizen::SPACE,
        seeds = [b"citizen", governance_pool.key().as_ref(), citizen.key().as_ref()],
        bump
    )]
    pub citizen_account: Account<'info, Citizen>,
    #[account(mut)]
    pub citizen: Signer<'info>,
    #[account(
        mut,
        seeds = [b"citizen_index", governance_pool.key().as_ref(), &(governance_pool.total_citizens / CitizenIndex::MAX_CITIZENS_PER_INDEX as u32).to_le_bytes()],
        bump
    )]
    pub citizen_index: Account<'info, CitizenIndex>,
    pub system_program: Program<'info, System>,
}

pub fn register_citizen(
    ctx: Context<RegisterCitizen>, 
    name: String, 
    region: u8, 
    age_group: u8, 
    other_demographic: u8
) -> Result<()> {
    let governance_pool = &mut ctx.accounts.governance_pool;
    let citizen_account = &mut ctx.accounts.citizen_account;
    let citizen_index = &mut ctx.accounts.citizen_index;

    require!(name.len() <= Citizen::MAX_NAME_LENGTH, GovernanceError::InvalidInput);
    require!(region < 8, GovernanceError::InvalidDemographic);
    require!(age_group < 5, GovernanceError::InvalidDemographic);
    require!(other_demographic < 4, GovernanceError::InvalidDemographic);

    citizen_account.name = name;
    citizen_account.governance_pool = governance_pool.key();
    citizen_account.is_eligible = true;
    citizen_account.last_participation = 0;
    citizen_account.region = region;
    citizen_account.age_group = age_group;
    citizen_account.other_demographic = other_demographic;
    citizen_account.is_initialized = true;

    if citizen_index.governance_pool == Pubkey::default() {
        citizen_index.governance_pool = governance_pool.key();
        citizen_index.citizens = Vec::new();
        citizen_index.count = 0;
    }

    // Add citizen to the index
    if citizen_index.citizens.len() >= CitizenIndex::MAX_CITIZENS_PER_INDEX {
        return Err(GovernanceError::CitizenIndexFull.into());
    }
    citizen_index.citizens.push(ctx.accounts.citizen.key());
    citizen_index.count += 1;

    governance_pool.total_citizens += 1;

    // Check if we need to create a new index
    if governance_pool.total_citizens % CitizenIndex::MAX_CITIZENS_PER_INDEX as u32 == 0 {
        governance_pool.total_citizen_indices += 1;
    }

    Ok(())
}