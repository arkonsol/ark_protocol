use anchor_lang::prelude::*;

#[error_code]
pub enum GovernanceError {
    #[msg("Citizen already registered")]
    CitizenAlreadyRegistered,
    #[msg("Assembly already active")]
    AssemblyAlreadyActive,
    #[msg("Not enough citizens for selection")]
    NotEnoughCitizens,
    #[msg("Not a member of the current assembly")]
    NotAssemblyMember,
    #[msg("Proposal already exists")]
    ProposalAlreadyExists,
    #[msg("Invalid vote")]
    InvalidVote,
    #[msg("Proposal not active")]
    ProposalNotActive,
    #[msg("Assembly term has ended")]
    AssemblyTermEnded,
    #[msg("Citizen not eligible")]
    CitizenNotEligible,
    #[msg("Invalid assembly size")]
    InvalidAssemblySize,
    #[msg("Invalid term length")]
    InvalidTermLength,
    #[msg("Random selection in progress")]
    RandomSelectionInProgress,
    #[msg("Invalid demographic quotas")]
    InvalidQuotas,
    #[msg("Invalid demographic information")]
    InvalidDemographic,
    #[msg("Citizen index is full")]
    CitizenIndexFull,
    #[msg("Invalid data")]
    InvalidInput,
    #[msg("Voting has not yet started")]
    VotingNotStarted,
    #[msg("Voting has not ended")]
    VotingEnded,
    #[msg("Invalid proposal")]
    InvalidProposal,
    #[msg("Overflow error")]
    Overflow,
    #[msg("Supply has been exceeded")] 
    ExceedsSupply,
    #[msg("Account is missing")]
    MissingAccount,
    #[msg("Account required is missing")]
    MissingRequiredAccount,
    #[msg("Mint is not valid")]
    InvalidMint,
    #[msg("Miissing NFT config")]
    MissingNFTConfig,
    #[msg("Missing spl config")]
    MissingSPLConfig,
    #[msg("No Junta token specified")]
    NoTokenSpecified,
    #[msg("Insufficient tokens to join junta")]
    InsufficientTokens,
    #[msg("Already a member of this junta")]
    AlreadyMember,
    #[msg("Arithmetic Error")]
    ArithmeticError,
    #[msg("Invite expired")]
    InviteExpired,
    #[msg("Invite used")]
    InviteAlreadyUsed,
    #[msg("Invite invalid")]
    InvalidInvite,
    #[msg("Invalid authority")]
    Unauthorized
}