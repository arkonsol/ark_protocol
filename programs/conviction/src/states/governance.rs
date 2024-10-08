use anchor_lang::prelude::*;

#[account]
pub struct Governance {
    pub name: String,
    pub description: String,
    pub creator: Pubkey,
    pub governance_token_mint: Pubkey,
    pub nft_mint: Option<Pubkey>,
    pub spl_mint: Option<Pubkey>,
    pub sbt_mint: Option<Pubkey>,
    pub nft_minted: u32,
    pub spl_minted: u32,
    pub sbt_minted: u32,
    pub nft_symbol: String,
    pub spl_symbol: String,
    pub symbol: String,
    pub approval_threshold: u64,
    pub resources: u64,
    pub total_proposals: u64,
    pub total_active_proposals: u64,
    pub total_members: u64,
    pub min_stake_amount: u64,
    pub collection_price: u64,
    pub total_spl_token_supply: u64,
    pub total_nft_token_supply: u64,
    pub total_sbt_token_supply: u64,
    pub bump: u8,
}

impl Governance {
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_OFFICERS: usize = 10;
    pub const MAX_DECREES: usize = 100;
    pub const SPL_PREFIX_SEED: &'static [u8] = b"spl";
    pub const NFT_PREFIX_SEED: &'static [u8] = b"nft";

    pub const MAX_NAME_LEN: usize = 32;          
    pub const MAX_DESCRIPTION_LEN: usize = 128; 
    pub const MAX_SYMBOL_LEN: usize = 12;

    pub const SPACE: usize = 8 + // discriminator
        4 + Self::MAX_NAME_LEN + // name (4 bytes for length + name content)
        4 + Self::MAX_DESCRIPTION_LEN + // description (4 bytes for length + description content)
        4 + Self::MAX_SYMBOL_LEN + // symbol (4 bytes for length + symbol content)
        32 + // creator
        32 + // governance_token_mint
        33 + // nft_token_mint
        33 + // spl_token_mint
        33 + // sbt token mint
        4 + // nft minted
        4 + // spl minted
        4 + // sbt minted
        8 + // approval_threshold
        8 + // total_proposals
        8 + // total_members
        8 + // min_stake_amount
        8 + // collection_price
        8 + // spl total token supply
        8 + // nft total token supply
        8 + // sbt total token supply
        1;  // bump (1 byte for the bump seed)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeGovernanceArgs {
    pub name: String,
    pub description: String,
    pub nft_symbol: String,
    pub spl_symbol: String,
    pub nft_supply: u64,
    pub spl_supply: u64,
    pub approval_threshold: u64,
    pub min_stake_amount: u64,
    pub collection_price: u64,
    pub nft_config: Option<TokenConfig>,
    pub spl_config: Option<TokenConfig>,
    pub primary_governance_token: PrimaryGovernanceToken,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenConfig {
    pub token_type: GovernanceTokenType,
    pub custom_mint: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GovernanceTokenType {
    New,
    Existing,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PrimaryGovernanceToken {
    NFT,
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