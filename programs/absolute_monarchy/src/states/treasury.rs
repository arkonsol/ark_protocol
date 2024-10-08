use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub balance: u64,
    pub taxes_collected: u64,
    pub last_collection_date: i64,
    pub royal_expenses: u64,
    pub operational_expenses: u64,
    pub military_funding: u64,
    pub bump: u8
}

impl Treasury {
    pub const SPACE: usize = 8 +  // discriminator
        8 +  // balance (u64)
        8 +  // taxes_collected (u64)
        8 +  // last_collection_date (i64)
        8 +  // royal_expenses (u64)
        8 +  // operational_expenses (u64)
        8 +    // military_funding (u64)
        1;

}