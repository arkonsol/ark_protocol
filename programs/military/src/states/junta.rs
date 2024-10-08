use anchor_lang::prelude::*;

#[account]
pub struct Junta {
    pub name: String,
    pub leader: Pubkey,
    pub officers: Vec<Pubkey>,
    pub resources: u64,
    pub decrees: Vec<Pubkey>,
    pub dissent_level: u8,
    pub control_level: u8,
    pub governance_token_mint: Pubkey,
    pub martial_law_active: bool,
    pub is_overthrown: bool,
    pub spl_symbol: String,
    pub nft_symbol: String,
    pub symbol: String,
    pub total_spl_token_supply: u64,
    pub total_nft_token_supply: u64,
    pub total_sbt_token_supply: u64,
    pub spl_minted: u64,
    pub nft_minted: u64,
    pub sbt_minted: u64,
    pub collection_price: u64,
    pub support_threshold: u8,
    pub spl_mint: Option<Pubkey>,
    pub nft_mint: Option<Pubkey>,
    pub sbt_mint: Option<Pubkey>,
    pub total_subjects: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeJuntaArgs {
    pub name: String,
    pub supply: u32,
    pub symbol: String,
    pub support_threshold: u8,
    pub collection_price: u64,
    pub nft_config: Option<JuntaTokenConfig>,
    pub spl_config: Option<JuntaTokenConfig>,
    pub nft_symbol: String,
    pub spl_symbol: String,
    pub nft_supply: u64,
    pub spl_supply: u64,
    pub primary_junta_token: PrimaryJuntaToken,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeSbtArgs {
    pub name: String,
    pub supply: u32,
    pub symbol: String,
    pub transferrable: bool,
    pub uri: String,
}

impl Junta {
    pub const PREFIX_SEED: &'static [u8] = b"junta";
    pub const SPL_PREFIX_SEED: &'static [u8] = b"spl";
    pub const NFT_PREFIX_SEED: &'static [u8] = b"nft";
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_OFFICERS: usize = 10;
    pub const MAX_DECREES: usize = 50;
    pub const MAX_SYMBOL_LENGTH: usize = 10;

    pub const SPACE: usize = 8 + // discriminator
        4 + Self::MAX_NAME_LENGTH + // name
        32 + // leader
        4 + (32 * Self::MAX_OFFICERS) + // officers
        8 + // resources
        4 + (32 * Self::MAX_DECREES) + // decrees
        1 + // dissent_level
        1 + // control_level
        33 + // governance_token_mint (Option<Pubkey>)
        1 + // martial_law_active
        1 + // is_overthrown
        4 + Self::MAX_SYMBOL_LENGTH + // spl_symbol
        4 + Self::MAX_SYMBOL_LENGTH + // nft_symbol
        4 + Self::MAX_SYMBOL_LENGTH + // symbol
        8 + // total_spl_token_supply
        8 + // total_nft_token_supply
        8 + // spl_minted
        8 + // nft_minted
        8 + // total subjects
        8 + // collection_price
        8 + // sbt minted
        8 + // total sbt supply
        33 + // sbt mint
        1 + // support_threshold
        33 + // spl_mint (Option<Pubkey>)
        33 + // nft_mint (Option<Pubkey>)
        1; // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct JuntaTokenConfig {
    pub token_type: JuntaTokenType,
    pub token_mint: Pubkey,
}

#[derive(Default)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum JuntaTokenType {
    New,
    #[default]
    Existing,
}

#[derive(Default)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PrimaryJuntaToken {
    NFT,
    #[default]
    SPL,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintNftArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}