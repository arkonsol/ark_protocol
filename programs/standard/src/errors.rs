use anchor_lang::prelude::*;

#[error_code]
pub enum RouterError {
    #[msg("Too many governments registered.")]
    TooManyGovernments,
    #[msg("Decision was rejected by the government.")]
    DecisionRejected,
    #[msg("Government not found.")]
    GovernmentNotFound,
    #[msg("The escrow condition has already been fulfilled")]
    AlreadyFulfilled,
    #[msg("The escrow condition has not been fulfilled")]
    ConditionNotFulfilled,
    #[msg("The escrow has expired")]
    Expired,
    #[msg("The escrow has not yet expired")]
    NotExpired,
    #[msg("Verification failed")]
    VerificationFailed,
    #[msg("Unauthorized to perform this action")]
    Unauthorized,
    #[msg("Verification has already been revoked")]
    AlreadyRevoked,
    #[msg("Invalid Epoch Index")]
    InvalidEpochIndex,
    #[msg("Max Epochs Reached")]
    MaxEpochsReached
}

