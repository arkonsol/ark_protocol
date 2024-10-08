use anchor_lang::prelude::*;

use crate::constants::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Time {
    FiveSeconds,
    TwentyFourHours,
    FourtyEightHours,
    OneWeek
}

impl Time {
    pub fn value(&self) -> i64 {
        match *self {
            Time::FiveSeconds => 5, // for testing purposes only
            Time::TwentyFourHours => ONE_DAY_IN_SECONDS,
            Time::FourtyEightHours => TWO_DAY_IN_SECONDS,
            Time::OneWeek => ONE_WEEK_IN_SECONDS,
        }
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Choice {
    Approve,
    Reject,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Status {
    Approved,
    Rejected,
    Voting
}