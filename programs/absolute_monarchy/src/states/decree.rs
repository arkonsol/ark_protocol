use anchor_lang::prelude::*;

#[account]
pub struct Decree {
    pub id: u64,
    pub text: String,
    pub decree_type: DecreeType,
    pub issued_at: i64,
    pub is_active: bool,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum DecreeType {
    Law,
    EconomicPolicy,
    MilitaryOrder,
    RoyalProclamation,
}

impl Decree {
    pub const MAXIMUM_TEXT_LENGTH: usize = 100; 

    pub const SPACE: usize = 8 +  // discriminator
        8 +  // id (u64)
        4 + Self::MAXIMUM_TEXT_LENGTH + // text (String)
        1 +  // decree_type (enum serialized as u8)
        8 +  // issued_at (i64)
        1 + //bump
        1;  // is_active (bool)
}