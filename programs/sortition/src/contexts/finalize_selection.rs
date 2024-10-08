use anchor_lang::prelude::*;
use crate::states::{assembly::Assembly, pagination::PaginationState, citizen_index::CitizenIndex, citizen::Citizen, governance::{GovernancePool, DemographicQuotas}};
use crate::error::GovernanceError;
use anchor_lang::solana_program::{sysvar::recent_blockhashes::ID, hash::hash};

#[derive(Accounts)]
pub struct FinalizeAssemblySelection<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,
    #[account(mut)]
    pub assembly: Account<'info, Assembly>,
    #[account(mut)]
    pub pagination_state: Account<'info, PaginationState>,
    #[account(
        mut,
        seeds = [
            b"citizen_index",
            governance_pool.key().as_ref(),
            &pagination_state.current_index.to_le_bytes()
        ],
        bump
    )]
    pub citizen_index: Account<'info, CitizenIndex>,
    #[account(mut, constraint = admin.key() == governance_pool.admin)]
    pub admin: Signer<'info>,
    /// CHECK: This is safe as we only read from this account
    #[account(
        mut,
        seeds = [b"recent-blockhashes"],
        bump,
        seeds::program = ID
    )]
    pub recent_blockhashes: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn finalize_assembly_selection<'info>(ctx: Context<'_, '_, 'info, 'info, FinalizeAssemblySelection<'info>>) -> Result<()> {
    let governance_pool = &mut ctx.accounts.governance_pool;
    let assembly = &mut ctx.accounts.assembly;
    let pagination_state = &mut ctx.accounts.pagination_state;
    let citizen_index = &ctx.accounts.citizen_index;

    require!(governance_pool.selection_in_progress, GovernanceError::RandomSelectionInProgress);

    let clock = Clock::get()?;
    let recent_blockhashes_data = ctx.accounts.recent_blockhashes.data.borrow();
    let most_recent_blockhash = &recent_blockhashes_data[12..44];

    // Combine previous seed, current slot, and most recent blockhash for randomness
    let new_seed = hash(&[
        &governance_pool.last_random_seed[..],
        &clock.slot.to_le_bytes(),
        most_recent_blockhash
    ].concat()).to_bytes();

    // Process a batch of citizens in this transaction
    let batch_size = 20; 
    let mut processed = 0;

    while processed < batch_size && pagination_state.current_citizen_in_index < citizen_index.citizens.len() as u32 {
        let citizen_pubkey = citizen_index.citizens[pagination_state.current_citizen_in_index as usize];

        if let Some(citizen_account) = find_and_verify_citizen_account(ctx.program_id, &citizen_pubkey, ctx.remaining_accounts)? {
                if citizen_account.is_eligible && is_demographically_eligible(&citizen_account, &pagination_state.demographic_counts, &governance_pool.demographic_quotas) {
                    // We'll use deterministic randomness to generate a unique random number for each citizen
                    // Using the first 8 bytes of the hash as a u64, then convert to a float between 0 and 1
                    let random_number = generate_random_number(&new_seed, pagination_state.current_citizen_in_index, &citizen_pubkey);
                    if random_number < calculate_selection_probability(&citizen_account, &pagination_state.demographic_counts, &governance_pool.demographic_quotas) {
                        pagination_state.selected_citizens.push(citizen_pubkey);
                        update_demographic_counts(&mut pagination_state.demographic_counts, &citizen_account);
                        
                        if pagination_state.selected_citizens.len() >= governance_pool.assembly_size as usize {
                            return finalize_selection(governance_pool, assembly, pagination_state);
                        }
                    }
                }
            
        }
        
        pagination_state.current_citizen_in_index += 1;
        processed += 1;
    }

    // Check if we've processed all citizens in the current index
    if pagination_state.current_citizen_in_index >= citizen_index.citizens.len() as u32 {
        pagination_state.current_index += 1;
        pagination_state.current_citizen_in_index = 0;
        
        // Check if we've processed all indices
        if pagination_state.current_index >= governance_pool.total_citizen_indices {
            finalize_selection(governance_pool, assembly, pagination_state)?;
        }
    }

    // Update the random seed for the next batch
    governance_pool.last_random_seed = new_seed;

    Ok(())
}

fn finalize_selection(
    governance_pool: &mut Account<GovernancePool>,
    assembly: &mut Account<Assembly>,
    pagination_state: &PaginationState,
) -> Result<()> {
    assembly.members = pagination_state.selected_citizens.clone();
    governance_pool.selection_in_progress = false;

    emit!(AssemblySelected {
        governance_pool: governance_pool.key(),
        assembly: assembly.key(),
        member_count: assembly.members.len() as u32,
    });

    Ok(())
}

fn find_and_verify_citizen_account<'a>(
    program_id: &Pubkey,
    citizen_pubkey: &Pubkey,
    remaining_accounts: &'a [AccountInfo<'a>]
) -> Result<Option<Account<'a, Citizen>>> {
    for account_info in remaining_accounts {
        if account_info.key() == *citizen_pubkey {
            return Ok(Some(Account::try_from(account_info)?));
        }
    }
    Ok(None)
}

#[event]
pub struct AssemblySelected {
    governance_pool: Pubkey,
    assembly: Pubkey,
    member_count: u32
}

fn is_demographically_eligible(citizen: &Citizen, current_counts: &DemographicQuotas, quotas: &DemographicQuotas) -> bool {
    current_counts.regions[citizen.region as usize] < quotas.regions[citizen.region as usize] &&
    current_counts.age_groups[citizen.age_group as usize] < quotas.age_groups[citizen.age_group as usize] &&
    current_counts.other_demographic[citizen.other_demographic as usize] < quotas.other_demographic[citizen.other_demographic as usize]
}

fn calculate_selection_probability(citizen: &Citizen, current_counts: &DemographicQuotas, quotas: &DemographicQuotas) -> u64 {
    const SCALE_FACTOR: u64 = 10_000;
    const REGION_WEIGHT: u64 = 4_000; // 40%
    const AGE_WEIGHT: u64 = 3_000;    // 30%
    const OTHER_WEIGHT: u64 = 3_000;  // 30%

    let region_factor = calculate_factor(
        quotas.regions[citizen.region as usize],
        current_counts.regions[citizen.region as usize],
        REGION_WEIGHT
    );
    let age_factor = calculate_factor(
        quotas.age_groups[citizen.age_group as usize],
        current_counts.age_groups[citizen.age_group as usize],
        AGE_WEIGHT
    );
    let other_factor = calculate_factor(
        quotas.other_demographic[citizen.other_demographic as usize],
        current_counts.other_demographic[citizen.other_demographic as usize],
        OTHER_WEIGHT
    );
    
    region_factor.saturating_add(age_factor).saturating_add(other_factor)
}

fn calculate_factor(quota: u32, current_count: u32, weight: u64) -> u64 {
    if quota == 0 {
        return 0;
    }
    (quota.saturating_sub(current_count) as u64)
        .saturating_mul(weight)
        .saturating_div(quota as u64)
}

fn update_demographic_counts(counts: &mut DemographicQuotas, citizen: &Citizen) {
    counts.regions[citizen.region as usize] += 1;
    counts.age_groups[citizen.age_group as usize] += 1;
    counts.other_demographic[citizen.other_demographic as usize] += 1;
}

fn generate_random_number(seed: &[u8; 32], index: u32, pubkey: &Pubkey) -> u64 {
    // Convert the index to a 4-byte array and concatenate all inputs
    let input_data = [
        seed.as_ref(),
        &index.to_le_bytes(),
        pubkey.as_ref()
    ].concat();
    
    // Perform Keccak hash on the concatenated input
    let hash = hash(&input_data).to_bytes();
    
    // Take the first 8 bytes of the hash and convert them to a u64
    u64::from_le_bytes(hash[0..8].try_into().unwrap())
}

// fn is_demographically_eligible(citizen: &Citizen, current_counts: &DemographicQuotas, quotas: &DemographicQuotas) -> bool {
//     current_counts.regions[citizen.region as usize] < quotas.regions[citizen.region as usize] &&
//     current_counts.age_groups[citizen.age_group as usize] < quotas.age_groups[citizen.age_group as usize] &&
//     current_counts.other_demographic[citizen.other_demographic as usize] < quotas.other_demographic[citizen.other_demographic as usize]
// }

// fn calculate_selection_probability(citizen: &Citizen, current_counts: &DemographicQuotas, quotas: &DemographicQuotas) -> f64 {
//     let region_factor = (quotas.regions[citizen.region as usize] - current_counts.regions[citizen.region as usize]) as f64 / quotas.regions[citizen.region as usize] as f64;
//     let age_factor = (quotas.age_groups[citizen.age_group as usize] - current_counts.age_groups[citizen.age_group as usize]) as f64 / quotas.age_groups[citizen.age_group as usize] as f64;
//     let other_factor = (quotas.other_demographic[citizen.other_demographic as usize] - current_counts.other_demographic[citizen.other_demographic as usize]) as f64 / quotas.other_demographic[citizen.other_demographic as usize] as f64;
    
//     (region_factor + age_factor + other_factor) / 3.0
// }

// fn update_demographic_counts(counts: &mut DemographicQuotas, citizen: &Citizen) {
//     counts.regions[citizen.region as usize] += 1;
//     counts.age_groups[citizen.age_group as usize] += 1;
//     counts.other_demographic[citizen.other_demographic as usize] += 1;
// }