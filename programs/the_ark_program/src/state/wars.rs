// wars.rs
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Wars {
    pub ongoing_wars: Vec<War>,
    pub past_wars: Vec<War>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct War {
    pub participants: Vec<Pubkey>,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub winner: Option<Pubkey>,
    pub war_type: WarType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum WarType {
    CivilWar,
    TerritorialDispute,
    RevolutionaryWar,
    // Add more as needed
}
