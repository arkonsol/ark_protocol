use anchor_lang::prelude::*;

#[account]
pub struct Circle {
    pub name: String,
    pub members: Vec<Pubkey>,
    pub proposals: Vec<Pubkey>,
    pub description: String,
    pub circle_type: CircleType,
    pub created_at: i64,
    pub updated_at: i64,
    pub nft_symbol: String,
    pub spl_symbol: String,
    pub collection_price: u64,
    pub resources: u64,
    pub nft_minted: u32,
    pub total_nft_token_supply: u64,
    pub spl_minted: u32,
    pub total_spl_token_supply: u64,
    pub sbt_minted: u32,
    pub total_sbt_token_supply: u64,
    pub spl_mint: Option<Pubkey>,
    pub nft_mint: Option<Pubkey>,
    pub sbt_mint: Option<Pubkey>,
    pub governance_token_mint: Pubkey,
    pub bump: u8,
}

// circle.description = args.description;
// circle.circle_type = args.circle_type;
// circle.created_at = clock.unix_timestamp;
// circle.updated_at = clock.unix_timestamp;

impl Circle {
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_DESCRIPTION_LENGTH: usize = 200;
    pub const MAX_MEMBERS: usize = 20;
    pub const MAX_PROPOSALS: usize = 50;
    pub const MAX_SYMBOL_LENGTH: usize = 10;
    pub const SPL_PREFIX_SEED: &'static [u8] = b"spl";
    pub const NFT_PREFIX_SEED: &'static [u8] = b"nft";

    pub const SPACE: usize = 8 + // discriminator
        4 + Self::MAX_NAME_LENGTH + // name
        4 + Self::MAX_DESCRIPTION_LENGTH + // description
        4 + (32 * Self::MAX_MEMBERS) + // members
        4 + (32 * Self::MAX_PROPOSALS) + // proposals
        1 + // circle_type (assuming it's an enum with 1 byte)
        8 + // created_at
        8 + // updated_at
        4 + Self::MAX_SYMBOL_LENGTH + // nft_symbol
        4 + Self::MAX_SYMBOL_LENGTH + // spl_symbol
        8 + // collection_price
        8 + // resources
        4 + // nft_minted
        8 + // total_nft_token_supply
        4 + // spl_minted
        8 + // total_spl_token_supply
        4 + // sbt_minted
        8 + // total_sbt_token_supply
        33 + // spl_mint
        33 + // nft_mint
        33 + // sbt_mint
        32 + // governance_token_mint
        1; // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CreateCircleArgs {
    pub name: String,
    pub description: String,
    pub circle_type: CircleType,
    pub nft_config: Option<CircleTokenConfig>,
    pub spl_config: Option<CircleTokenConfig>,
    pub nft_symbol: String,
    pub spl_symbol: String,
    pub nft_supply: u64,
    pub spl_supply: u64,
    pub collection_price: u64,
    pub primary_governance_token: PrimaryGovernanceToken,
}

#[derive(Default)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum CircleType {
    #[default]
    General,
    Project,
    Department,
}

#[account]
pub struct CircleMemberRecord {
    pub circle: Pubkey,
    pub member: Pubkey,
    pub joined_at: i64,
    pub bump: u8,
}

impl CircleMemberRecord {
    pub const SPACE: usize = 8 + // discriminator
        32 + // circle
        32 + // member
        8 +  // joined_at
        1;  // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CircleTokenConfig {
    pub token_type: CircleTokenType,
    pub token_mint: Pubkey,
}

#[derive(Default)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum CircleTokenType {
    New,
    #[default]
    Existing,
}

#[derive(Default)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PrimaryGovernanceToken {
    NFT,
    #[default]
    SPL,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeSbtArgs {
    pub name: String,
    pub supply: u32,
    pub symbol: String,
    pub transferrable: bool,
    pub uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MintNftArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}