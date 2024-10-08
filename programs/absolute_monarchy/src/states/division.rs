use anchor_lang::prelude::*;

#[account]
pub struct Division {
    pub name: String,
    pub manager: Pubkey,
    pub established_at: i64,
    pub last_transfer_at: i64,
    pub treasury: Pubkey,
    pub bump: u8, 
}

impl Division {
    pub const MAXIMUM_NAME_LENGTH: usize = 50;

    pub const SPACE: usize = 8 + // discriminator
        4 + Self::MAXIMUM_NAME_LENGTH + // name (String)
        32 + // manager (Pubkey)
        8 + // established_at (i64)
        8 + // last_transfer_at (i64)
        32 +  // treasury (Pubkey)
        1; // bump
}