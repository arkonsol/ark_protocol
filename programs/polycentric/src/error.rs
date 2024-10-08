use anchor_lang::prelude::*;

#[error_code]
pub enum GovernanceError {
    #[msg("Maximum number of assemblies reached")]
    MaxAssembliesReached,
    #[msg("Maximum number of policy areas reached")]
    MaxPolicyAreasReached,
    #[msg("Maximum number of assembly members reached")]
    MaxAssemblyMembersReached,
    #[msg("Maximum number of policy areas per assembly reached")]
    MaxPolicyAreasPerAssemblyReached,
    #[msg("Maximum number of assemblies per policy area reached")]
    MaxAssembliesPerPolicyAreaReached,
    #[msg("Maximum number of proposals per policy area reached")]
    MaxProposalsPerPolicyAreaReached,
    #[msg("Proposal is not in active state")]
    ProposalNotActive,
    #[msg("Proposal voting period has ended")]
    ProposalVotingEnded,
    #[msg("User has already voted on this proposal")]
    AlreadyVoted,
    #[msg("User is not a member of the required assembly")]
    NotAssemblyMember,
    #[msg("Proposal is not approved")]
    ProposalNotApproved,
    #[msg("Insufficient permissions to perform this action")]
    InsufficientPermissions,
    #[msg("Invalid governance pool")]
    InvalidGovernancePool,
    #[msg("Invalid assembly")]
    InvalidAssembly,
    #[msg("Invalid policy area")]
    InvalidPolicyArea,
    #[msg("Invalid proposal")]
    InvalidProposal,
    #[msg("Maximum treasuries reached")]
    MaxTreasuriesReached,
    #[msg("Not enough voting power")]
    InsufficientVotingPower,
    #[msg("Already a member")]
    AlreadyMember,
    #[msg("Task has been completed")]
    TaskAlreadyCompleted,
    #[msg("Task Not Found")]
    TaskNotFound,
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
    #[msg("Insufficient tokens")] 
    InsufficientTokens,
    #[msg("Invite expired")]
    InviteExpired,
    #[msg("Invite used")]
    InviteAlreadyUsed,
    #[msg("Invite invalid")]
    InvalidInvite,
    #[msg("Unauthorized")]
    Unauthorized
}