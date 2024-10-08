use anchor_lang::error_code;


#[error_code]
pub enum GovernanceError {
    #[msg("Circle already exists")]
    CircleAlreadyExists,
    #[msg("Member already exists")]
    MemberAlreadyExists,
    #[msg("Proposal already exists")]
    ProposalAlreadyExists,
    #[msg("Invalid vote")]
    InvalidVote,
    #[msg("Not a circle member")]
    NotCircleMember,
    #[msg("Not authorized")]
    NotAuthorized,
    #[msg("Proposal not active")]
    ProposalNotActive,
    #[msg("Account is missing")]
    MissingAccount,
    #[msg("Account required is missing")]
    MissingRequiredAccount,
    #[msg("SPL Mint is not valid")]
    InvalidSPLMint,
    #[msg("NFT Mint is not valid")]
    InvalidNFTMint,
    #[msg("Task is not valid")]
    InvalidTask,
    #[msg("Overflow error")]
    Overflow,
    #[msg("Supply has been exceeded")] 
    ExceedsSupply,
    #[msg("No governance token")] 
    NoGovernanceTokenSpecified,
    #[msg("Missing NFT config")] 
    MissingNFTConfig,
    #[msg("Missing SPL config")] 
    MissingSPLConfig,
    #[msg("Name is too long")] 
    NameTooLong,
    #[msg("Description is too long")] 
    DescriptionTooLong,
    #[msg("Missing Parent Circle")] 
    MissingParentCircle,
    #[msg("Invalid Parent Circle")] 
    InvalidParentCircle,
    #[msg("Circle is full")] 
    CircleFullyBanner,
    #[msg("Invite expired")]
    InviteExpired,
    #[msg("Invite used")]
    InviteAlreadyUsed,
    #[msg("Invite invalid")]
    InvalidInvite
}