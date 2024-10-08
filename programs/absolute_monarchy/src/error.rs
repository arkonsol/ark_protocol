use anchor_lang::prelude::*;

#[error_code]
pub enum AbsoluteMonarchyError {
    #[msg("Only the current Monarch can perform this action")]
    NotMonarch,
    #[msg("Insufficient funds in the royal treasury")]
    InsufficientFunds,
    #[msg("Insufficient tokens to become a member")]
    InsufficientTokens,
    #[msg("Invalid token account owner")]
    InvalidTokenAccountOwner,
    #[msg("Invalid treasury account")]
    InvalidTreasuryAccount,
    #[msg("Subject not found")]
    SubjectNotFound,
    #[msg("Monarchy already initialized")]
    AlreadyInitialized,
    #[msg("Invalid decree")]
    InvalidDecree,
    #[msg("War already declared against this enemy")]
    WarAlreadyDeclared,
    #[msg("Subject is not convicted")]
    SubjectNotConvicted,
    #[msg("Invalid noble title")]
    InvalidNobleTitle,
    #[msg("Territory already colonized")]
    TerritoryAlreadyColonized,
    #[msg("Invalid economic policy")]
    InvalidEconomicPolicy,
    #[msg("Invalid tax rate")]
    InvalidTaxRate,
    #[msg("Policy title is too long")]
    PolicyTitleTooLong,
    #[msg("Policy description is too long")]
    PolicyDescriptionTooLong,
    #[msg("Jurisdiction name is too long")]
    JurisdictionTooLong,
    #[msg("Not the policy owner")]
    NotPolicyOwner,
    #[msg("Invalid usage fee rate")]
    InvalidUsageFeeRate,
    #[msg("Invalid access level")]
    InvalidAccessLevel,
    #[msg("Privileged access has expired")]
    AccessExpired,
    #[msg("Transfer is not yet due")]
    TransferTooSoon,
    #[msg("Wrong treasury")]
    InvalidTreasury,
    #[msg("Missing required account")]
    MissingRequiredAccount,
    #[msg("Invalid SPL mint")]
    InvalidSPLMint,
    #[msg("Missing NFT configuration")]
    MissingNFTConfig,
    #[msg("Missing SPL configuration")]
    MissingSPLConfig,
    #[msg("No kingdom token specified")]
    NoKingdomTokenSpecified,
    #[msg("Invalid treasury owner")]
    InvalidTreasuryOwner,
    #[msg("Division name cannot be empty")]
    EmptyDivisionName,
    #[msg("Invalid monarch for this kingdom")]
    InvalidMonarch,
    #[msg("Monarch and kingdom accounts do not match")]
    MonarchKingdomMismatch,
    #[msg("Heir name cannot be empty")]
    EmptyHeirName,
    #[msg("This mint is not valid")]
    InvalidMint,
    #[msg("Supply has been exceeded")]
    ExceedsSupply,
    #[msg("Overflow error")]
    Overflow,
    #[msg("Invite expired")]
    InviteExpired,
    #[msg("Invite used")]
    InviteAlreadyUsed,
    #[msg("Invite invalid")]
    InvalidInvite
}