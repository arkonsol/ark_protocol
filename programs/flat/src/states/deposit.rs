use anchor_lang::prelude::*;

use crate::constants::*;


#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Deposit {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub deactivating: bool,
    pub deactivation_start: Option<i64>,
    pub created_at: i64,
}

impl Deposit {
    pub const LEN: usize = PUBLIC_KEY_LENGTH * 2 
        + 8 // amount
        + BOOL_LENGTH // bool
        + 1 // option
        + TIMESTAMP_LENGTH * 2;
}