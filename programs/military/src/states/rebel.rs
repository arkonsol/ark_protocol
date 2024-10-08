use anchor_lang::prelude::*;
use crate::states::citizen::Citizen;
use crate::errors::ErrorCode;

pub const MAX_REBELS: usize = 10;
#[account]
pub struct Rebel {
    pub rebels: [Option<Citizen>; MAX_REBELS], 
    pub count: u8,  
}

impl Rebel {
    pub const MAX_REBELS: usize = 10;
    
    pub const SIZE: usize = 8 +
        (Self::MAX_REBELS * std::mem::size_of::<Option<Citizen>>()) + // Size of rebels array
        1; // Size of count (u8)

    pub fn add_rebel(&mut self, citizen: Citizen) -> Result<()> {
        require!(self.count < MAX_REBELS as u8, ErrorCode::MaxRebelsReached);
        
        for rebel_slot in self.rebels.iter_mut() {
            if rebel_slot.is_none() {
                *rebel_slot = Some(citizen);
                self.count += 1;
                return Ok(());
            }
        }

        Err(ErrorCode::MaxRebelsReached.into())
    }

    pub fn remove_rebel(&mut self, target: Pubkey) -> Result<()> {
        for rebel_slot in self.rebels.iter_mut() {
            if let Some(rebel) = rebel_slot {
                if rebel.authority == target {
                    *rebel_slot = None; 
                    self.count -= 1;
                    return Ok(());
                }
            }
        }

        Err(ErrorCode::RebelNotFound.into())
    }
}
