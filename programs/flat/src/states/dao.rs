use anchor_lang::prelude::*;

use crate::constants::*;
use crate::states::{Poll, User};


#[account]
pub struct DAO {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub time: i64,
    pub threshold: u8,
    pub min_poll_tokens: u64,
    pub approved: u64,
    pub rejected: u64,
    pub created_at: i64,
    pub dao_bump: u8,
    pub vault_bump: u8,
    pub name: String,
    pub polls: Vec<Poll>,
    pub users: Vec<User>,
    pub total_members: u32, 
}

impl DAO {

    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2  // creator, mint 
        + 1 + 8  // threshold, time
        + 1  // threshold 51 => 100
        + 8 * 3  // approved, rejected, min_poll_tokens 
        + TIMESTAMP_LENGTH * 2  // time, created_at
        + BUMP_LENGTH * 2  // dao_bump, vault_bump
        + VECTOR_LENGTH_PREFIX * 2  // for polls and users vectors
        + STRING_LENGTH_PREFIX
        + MAX_DAO_NAME_LENGTH
        + 4;  // total_members (u32)

    // ... other methods ...

    pub fn total_deposits(&self) -> usize {
        self.users.iter().map(|user| user.deposits.len()).sum()
    }
    pub fn total_polls(&self) -> usize {
        self.polls.len()
    }

    pub fn total_deposit_amount(&self) -> u64 {
        self.users.iter().map(|user| {
            user.deposits.iter().map(|deposit| deposit.amount).sum::<u64>()
        }).sum()
    }

    pub fn total_votes(&self) -> usize {
        self.polls.iter().map(|poll| poll.votes.len()).sum()
    }
    
    pub fn reward_points(&mut self, poll_id: usize) {
        if let Some(poll) = self.polls.get(poll_id) {
            for vote in &poll.votes {
                if let Some(user) = self.users.iter_mut().find(|user| user.user == vote.user) {
                    user.points += vote.voting_power;
                }
            }
        }
    }
}