use anchor_lang::prelude::*;

use crate::constants::*;
use crate::states::{Status, Vote, Choice, DAO};

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Poll {
    pub creator: Pubkey,
    pub created_at: i64,
    pub executed: bool,
    pub status: Status,
    pub title: String,
    pub content: String,
    pub votes: Vec<Vote>,
}

impl Poll {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // creator 
        + TIMESTAMP_LENGTH // created_at
        + BOOL_LENGTH 
        + STRING_LENGTH_PREFIX * 2
        + MAX_TITLE_LENGTH
        + MAX_CONTENT_LENGTH
        + VECTOR_LENGTH_PREFIX; // bump

    pub fn is_approved(&self, dao: &DAO) -> bool {
        let mut approve_power = 0u64;
        let mut total_power = 0u64;

        for vote in &self.votes {
            total_power += vote.voting_power;
            if vote.choice == Choice::Approve {
                approve_power += vote.voting_power;
            }
        }

        let approval_percentage = (approve_power as f64 / total_power as f64) * 100.0;
        approval_percentage >= dao.threshold as f64
    }}