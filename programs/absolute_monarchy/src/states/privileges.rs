use anchor_lang::prelude::*;

#[account]
pub struct PrivilegedAccess {
    pub access_type: String,
    pub holder: Pubkey,
    pub granted_at: i64,
    pub expires_at: i64,
    pub usage_fee_rate: u8,
    pub access_level: u8,
    pub bump: u8
}

impl PrivilegedAccess {
    pub const MAXIMUM_ACCESS_TYPE_LENGTH: usize = 50;

    pub const SPACE: usize =
        8 + // discriminator
        4 + Self::MAXIMUM_ACCESS_TYPE_LENGTH + // access_type (String)
        32 + // holder (Pubkey)
        8 + // granted_at (i64)
        8 + // expires_at (i64)
        1 + // usage_fee_rate (u8)
        1 +  // access_level (u8)
        1;
}