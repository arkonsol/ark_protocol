use anchor_lang::error_code;


#[error_code]
pub enum ErrorCode {
    #[msg("Invalid authority.")]
    Unauthorized,
    #[msg("This proposal is not active. Try creating a new one.")]
    ProposalNotActive,
    #[msg("The voting period for this proposal is currently inactive.")]
    VotingPeriodInactive,
    #[msg("An overflow has occurred. Please try again.")]
    Overflow,
    #[msg("An invalid data was parsed. Please try again.")]
    InvalidProposalData,
    #[msg("Voting period has not yet concluded. Please try again later.")]
    VotingPeriodNotEnded,
    #[msg("This paramter name does not exist. Please try again.")]
    InvalidParameterName,
    #[msg("Missing NFT Config. Please try again.")]
    MissingNFTConfig,
    #[msg("Missing SPL Config. Please try again.")]
    MissingSPLConfig,
    #[msg("Atleast one type of governance token must be specified. Please try again.")]
    NoGovernanceTokenSpecified,
    #[msg("Missing required account to continue. Please try again.")]
    MissingRequiredAccount,
    #[msg("This SPL Mint is invalid. Please try again.")]
    InvalidSPLMint,
    #[msg("This NFT Mint is invalid. Please try again.")]
    InvalidNFTMint,
    #[msg("This mint is invalid. Please try again.")]
    InvalidMint,
    #[msg("You do not have up to this amount to unstake. Please try again later.")]
    InsufficientStake,
    #[msg("This stake is locked. Please try again later.")]
    StakeLocked,
    #[msg("This stake is below minimum amount. Please try again later.")]
    StakeTooLow,
    #[msg("Amount minted has exceeded the total supply. Please try again later.")]
    ExceedsSupply,
    #[msg("Insufficient voting power. Please try staking more tokens for more voting power.")]
    ExceedsAvailableVotingPower,
    #[msg("Insufficient amount to unstake. Please try staking more tokens for more voting power.")]
    InvalidNFTStake,
    #[msg("Insufficient tokens")] 
    InsufficientTokens,
    #[msg("NFT Mint not set")] 
    NftMintNotSet,
    #[msg("SPL mint not set")] 
    SplMintNotSet,
    #[msg("Invite expired")]
    InviteExpired,
    #[msg("Invite used")]
    InviteAlreadyUsed,
    #[msg("Invite invalid")]
    InvalidInvite
}