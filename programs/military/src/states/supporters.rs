use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

pub const MAX_SUPPORTERS: usize = 5; 
pub const SUPPORTERS_PER_LEVEL: usize = 5;
pub const MAX_SUPPORT_AMOUNTS: usize = 5; 

#[account]
pub struct Supporters {
    pub supporters: [Option<Pubkey>; MAX_SUPPORTERS],
    pub support_amounts: [(Pubkey, u64); MAX_SUPPORT_AMOUNTS],
    pub count: u8,
    pub bump: u8,
}

impl Supporters {
    pub const SIZE: usize = 8 + // Discriminator
    (MAX_SUPPORTERS * (1 + 32)) + // supporters array
    (MAX_SUPPORT_AMOUNTS * (32 + 8)) + // support_amounts array
    1 + // count
    1; // bump

    pub fn add_supporter(&mut self, supporter: Pubkey) -> Result<()> {
        require!(self.count < MAX_SUPPORTERS as u8, ErrorCode::MaxSupportersReached);
        
        for slot in self.supporters.iter_mut() {
            if slot.is_none() {
                *slot = Some(supporter);
                self.count += 1;
                return Ok(());
            }
        }

        Err(ErrorCode::MaxSupportersReached.into())
    }

    pub fn is_supporter(&self, pubkey: &Pubkey) -> bool {
        self.supporters.iter().any(|s| s.as_ref() == Some(pubkey))
    }

    pub fn get_support_amount(&self, supporter: &Pubkey) -> u64 {
        self.support_amounts
            .iter()
            .find(|(pubkey, _)| pubkey == supporter)
            .map(|(_, amount)| *amount)
            .unwrap_or(0)
    }

    pub fn update_support_amount(&mut self, supporter: Pubkey, amount: u64) -> Result<()> {
        // First, try to find and update an existing entry
        for entry in self.support_amounts.iter_mut() {
            if entry.0 == supporter {
                entry.1 = amount;
                return Ok(());
            }
        }
        
        // If not found, try to add a new entry
        for entry in self.support_amounts.iter_mut() {
            if entry.0 == Pubkey::default() {
                *entry = (supporter, amount);
                return Ok(());
            }
        }
        
        // If we get here, the array is full
        Err(ErrorCode::SupportAmountsArrayFull.into())
    }
}