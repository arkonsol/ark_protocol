use anchor_lang::prelude::*;

#[account]
pub struct Subject {
    pub key: Pubkey,
    pub name: String,
    pub role: String,
    pub nobility_title: Option<String>,
    pub loyalty: u8,
    pub wealth: u64,
    pub is_convicted: bool,
    pub jurisdiction: String,
    pub appointed_at: i64,
    pub bump: u8

}

impl Subject {
    pub const MAXIMUM_NAME_LENGTH: usize = 50;
    pub const MAXIMUM_ROLE_LENGTH: usize = 50;
    pub const MAXIMUM_NOBILITY_TITLE_LENGTH: usize = 50;
    pub const MAXIMUM_JURISDICTION_LENGTH: usize = 50;

    pub const SPACE: usize = 8 +  // discriminator
        32 + // key (Pubkey)
        4 + Self::MAXIMUM_NAME_LENGTH + // name (String)
        4 + Self::MAXIMUM_ROLE_LENGTH + // role (String)
        1 + 4 + Self::MAXIMUM_NOBILITY_TITLE_LENGTH + // nobility_title (Option<String>)
        1 + // loyalty (u8)
        8 + // wealth (u64)
        1 + // is_convicted (bool)
        4 + Self::MAXIMUM_JURISDICTION_LENGTH + // jurisdiction (String)
        8 + // appointed_at
        1;
}