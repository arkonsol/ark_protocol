use anchor_lang::prelude::*;

#[account]
pub struct Monarch {
    pub authority: Pubkey,
    pub name: String,
    pub divine_mandate: String,
    pub coronation_date: i64,
    pub abdication_date: Option<i64>,
    pub decrees_issued: u64,
    pub wars_declared: u64,
    pub royal_judgments: u64,
    pub economic_policies_set: u64,
    pub pardons_granted: u64,
    pub policies_implemented: u64,
    pub bump: u8,
}

impl Monarch {
    pub const MAXIMUM_NAME_LENGTH: usize = 50;
    pub const MAXIMUM_DIVINE_MANDATE_LENGTH: usize = 100;

    pub const SPACE: usize = 8 +  // discriminator
        32 + // authority (Pubkey)
        4 + Self::MAXIMUM_NAME_LENGTH + // name (String)
        4 + Self::MAXIMUM_DIVINE_MANDATE_LENGTH + // divine_mandate (String)
        8 + // coronation_date (i64)
        4 + 8 + // abdication_date (i64)
        8 + // decrees_issued (u64)
        8 + // wars_declared (u64)
        8 + // royal_judgments (u64)
        8 +  // economic_policies_set (u64)
        8 + // pardons_granted
        8 + // policies_implemented
        1; // bump
}