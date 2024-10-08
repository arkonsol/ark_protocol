use anchor_lang::prelude::*;

#[account]
pub struct Noble {
    pub subject: Pubkey,
    pub title: String,
    pub granted_at: i64,
    pub bump: u8
}

impl Noble {
    pub const MAXIMUM_TITLE_LENGTH: usize = 50;

    pub const SPACE: usize = 8 + // discriminator
        32 + // subject
        4 + Self::MAXIMUM_TITLE_LENGTH + // title
        8 + // granted_at
        1; // bump
}