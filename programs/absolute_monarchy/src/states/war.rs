use anchor_lang::prelude::*;

#[account]
pub struct War {
    pub enemy_program: Pubkey,
    pub reason: String,
    pub declared_at: i64,
    pub is_active: bool,
    pub battles_won: u64,
    pub battles_lost: u64,
    pub bump: u8
}

impl War {
    pub const MAXIMUM_REASON_LENGTH: usize = 200;

    pub const SPACE: usize = 8 + // discriminator
        32 + // enemy_program (Pubkey)
        4 + Self::MAXIMUM_REASON_LENGTH + // reason (String)
        8 + // declared_at (i64)
        1 + // is_active (bool)
        8 + // battles_won (u64)
        8 +  // battles_lost (u64)
        1;
}