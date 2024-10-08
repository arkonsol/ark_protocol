use anchor_lang::prelude::*;

#[account]
pub struct Decree {
    pub issuer: Pubkey,
    pub content: String,
    pub issued_at: i64,
    pub bump: u8
}

impl Decree {
    pub const MAX_CONTENT_LENGTH: usize = 1000;
}