use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Too many officers. Maximum number is 10.")]
    TooManyOfficers,
    #[msg("The maximum amount of 10 rebels have been reached.")]
    MaxRebelsReached,
    #[msg("The maximum amount of 100 rebels have been reached.")]
    MaxSupportersReached,
    #[msg("This rebel does not exist.")]
    RebelNotFound,
    #[msg("e started specified is invalid.")]
    InvalidTarget,
    #[msg("More rebels are needed for a rebellion to start.")]
    NotEnoughRebels,
    #[msg("Not enough resources.")]
    InsufficientResources,
    #[msg("Not a valid supporter.")]
    InvalidSupporter,
    #[msg("Not enough supporters.")]
    InsufficientSupporters,
    #[msg("The supporter is a dissident.")]
    SupporterIsDissident,
    #[msg("Too many decrees. Maximum number is 100.")]
    TooManyDecrees,
    #[msg("Not the valid new leader")]
    InvalidLeader,
    #[msg("Failed to mint NFT")]
    MintFailed,
    #[msg("Supply has reached the maximum limit")]
    SupplyReached,
    #[msg("Unauthorized Signer")]
    UnauthorizedSigner,
    #[msg("Same user accounts ")]
    SameUserAccounts,
    #[msg("Overflow error")]
    Overflow,
    #[msg("Underflow error")]
    Underflow,
    #[msg("Exceeeds maximum supply ")]
    ExceedsSupply,
    #[msg("Invalid Junta Vault ")]
    InvalidJuntaVault,
    #[msg("The maximum size of support has been met")]
    SupportAmountsArrayFull,
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
    NoJuntaTokenSpecified,
    #[msg("Insufficient tokens to join junta")]
    InsufficientTokens,
    #[msg("Already a member of this junta")]
    AlreadyMember,
    #[msg("This invite has already been used")]
    InviteAlreadyUsed,
    #[msg("This invite has expired")]
    InviteExpired,
    #[msg("Invalid invite for this Junta")]
    InvalidInvite,
    #[msg("Unauthorized to perform this action")]
    Unauthorized,
    #[msg("Arithmetic error occurred")]
    ArithmeticError,
}