use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[account]
pub struct Citizen {
    pub name: String,
    pub governance_pool: Pubkey,
    pub is_eligible: bool,
    pub last_participation: i64,
    pub region: u8, 
    pub age_group: u8, 
    pub other_demographic: u8,
    pub is_initialized: bool,
    pub bump: u8,
}

impl Citizen {
    pub const MAX_NAME_LENGTH: usize = 50;

    pub const SPACE: usize = 8 + // discriminator
        4 + Self::MAX_NAME_LENGTH + // name
        32 + // governance_pool
        1 + // is_eligible
        8 + // last_participation
        1 +
        1 +
        1 +
        1 +
        1; // bump

    pub fn try_load_by_pubkey<'info>(
        program_id: &Pubkey,
        account_info: &'info AccountInfo<'info>
    ) -> Result<Option<Account<'info, Self>>> {
        if account_info.owner != program_id {
            return Ok(None);
        }

        match Account::<Self>::try_from(account_info) {
            Ok(account) => {
                // Assuming Citizen has an is_initialized field
                if account.is_initialized {
                    Ok(Some(account))
                } else {
                    Ok(None)
                }
            },
            Err(_) => Ok(None),
        }
    }
}