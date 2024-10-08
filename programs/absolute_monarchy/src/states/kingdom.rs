use anchor_lang::prelude::*;

#[account]
pub struct Kingdom {
    pub name: String,
    pub description: String,
    pub creator: Pubkey,
    pub monarch: Pubkey,
    pub monarch_name: String,
    pub primary_token_mint: Pubkey,
    pub nft_mint: Option<Pubkey>,
    pub spl_mint: Option<Pubkey>,
    pub sbt_mint: Option<Pubkey>,
    pub nft_symbol: String,
    pub spl_symbol: String,
    pub symbol: String,
    pub nft_minted: u64,
    pub spl_minted: u64,
    pub sbt_minted: u64,
    pub royal_treasury: u64,
    pub total_decrees: u64,
    pub total_active_decrees: u64,
    pub total_subjects: u64,
    pub min_loyalty_amount: u64,
    pub total_spl_token_supply: u64,
    pub total_nft_token_supply: u64,
    pub total_sbt_token_supply: u64,
    pub established_at: i64,
    pub wars_declared: u64,
    pub royal_judgments: u64,
    pub economic_policies_set: u64,
    pub pardons_granted: u64,
    pub policies_implemented: u64,
    pub divisions: Vec<Pubkey>,
    pub membership_tokens_threshold: u64,
    pub officials_appointed: Vec<Pubkey>,
    pub nobles: Vec<Pubkey>,
    pub collection_price: u64,
    pub bump: u8,
}

impl Kingdom {
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_SYMBOL_LENGTH: usize = 20;
    pub const MAX_DESCRIPTION_LENGTH: usize = 200;
    pub const MAX_DIVISIONS: usize = 5;
    pub const MAX_OFFICIALS: usize = 5;
    pub const MAX_NOBLES: usize = 5;

    pub const SPL_PREFIX_SEED: &'static [u8] = b"spl";
    pub const NFT_PREFIX_SEED: &'static [u8] = b"nft";
    pub const SBT_PREFIX_SEED: &'static [u8] = b"sbt";

    pub const SPACE: usize = 8 + // discriminator
        4 + Self::MAX_NAME_LENGTH +
        4 + Self::MAX_DESCRIPTION_LENGTH +
        32 + // creator
        32 + // monarch
        4 + Self::MAX_NAME_LENGTH +
        4 + Self::MAX_SYMBOL_LENGTH + // nft
        4 + Self::MAX_SYMBOL_LENGTH + // symbol
        4 + Self::MAX_SYMBOL_LENGTH + // spl token
        32 + // primary_token_mint
        33 + // nft_mint
        33 + // spl_mint
        33 + // sbt_mint
        4 + // nft_minted
        4 + // spl_minted
        4 + // sbt_minted
        8 + // royal_treasury
        8 + // total_decrees
        8 + // total_active_decrees
        8 + // total_subjects
        8 + // min_loyalty_amount
        8 + // total_spl_token_supply
        8 + // total_nft_token_supply
        8 + // total_sbt_token_supply
        8 + // collection_price
        8 + // established_at
        8 + // wars declared
        8 + // royal judgments
        8 + // economic policies
        8 + // pardons
        8 + // policies set
        4 + (32 * Self::MAX_DIVISIONS) + // divisions
        4 + (32 * Self::MAX_OFFICIALS) + // offficials
        4 + (32 * Self::MAX_NOBLES) + // nobles
        8 + // membership token threshold
        1;  // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeKingdomArgs {
    pub name: String,
    pub description: String,
    pub monarch_name: String,
    pub divine_mandate: String,
    pub collection_price: u64,
    pub nft_supply: u64,
    pub spl_supply: u64,
    pub royal_decree_threshold: u64,
    pub min_loyalty_amount: u64,
    pub membership_tokens_threshold: u64,
    pub knighthood_price: u64,
    pub nft_config: Option<TokenConfig>,
    pub spl_config: Option<TokenConfig>,
    pub primary_kingdom_token: PrimaryKingdomToken,
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenConfig {
    pub token_type: KingdomTokenType,
    pub custom_mint: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum KingdomTokenType {
    New,
    Existing,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PrimaryKingdomToken {
    NFT,
    SPL,
}