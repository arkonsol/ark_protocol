use anchor_lang::prelude::*;
use the_ark_program::interface::GovernmentTypes;
#[account]
pub struct RouterState {
    pub authority: Pubkey,
    pub governments: Vec<GovernmentEntry>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GovernmentEntry {
    pub government_type: GovernmentTypes,
    pub program_id: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum GovernmentInstruction {
    MakeDecision,
}

#[error_code]
pub enum RouterError {
    #[msg("Too many governments registered")]
    TooManyGovernments,
    #[msg("Government not found")]
    GovernmentNotFound,
    #[msg("Decision rejected by the government")]
    DecisionRejected,
}