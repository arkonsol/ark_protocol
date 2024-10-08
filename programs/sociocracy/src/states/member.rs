use anchor_lang::prelude::*;

#[account]
pub struct Member {
    pub name: String,
    pub circles: Vec<Pubkey>,
    pub bump: u8,
}

impl Member {
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_CIRCLES: usize = 5;

    pub const SPACE: usize = 8 + // discriminator
        4 + Self::MAX_NAME_LENGTH + // name
        4 + (32 * Self::MAX_CIRCLES) + // circles
        1;
}