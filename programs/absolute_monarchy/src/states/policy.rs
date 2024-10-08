use anchor_lang::prelude::*;

#[account]
pub struct Policy {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub target_jurisdiction: String,
    pub implemented_at: i64,
    pub last_updated_at: i64,
    pub is_active: bool,
    pub monarch: Pubkey,
}

impl Policy {
    pub const MAXIMUM_TITLE_LENGTH: usize = 100;
    pub const MAXIMUM_DESCRIPTION_LENGTH: usize = 300;
    pub const MAXIMUM_JURISDICTION_LENGTH: usize = 50;

    pub const SPACE: usize = 8 +  // discriminator
        8 +  // id (u64)
        4 + Self::MAXIMUM_TITLE_LENGTH + // title (String)
        4 + Self::MAXIMUM_DESCRIPTION_LENGTH + // description (String)
        4 + Self::MAXIMUM_JURISDICTION_LENGTH + // target_jurisdiction (String)
        8 +  // implemented_at (i64)
        8 +  // last_updated_at (i64)
        1 +  // is_active (bool)
        32 +   // monarch (Pubkey)
        1;
}