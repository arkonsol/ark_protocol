use anchor_lang::prelude::*;

#[account]
pub struct GovernancePool {
    pub admin: Pubkey,
    pub total_citizens: u32,
    pub assembly_size: u8,
    pub assembly_term: i64,
    pub current_assembly: Pubkey,
    pub last_random_seed: [u8; 32],
    pub selection_in_progress: bool,
    pub total_citizen_indices: u32,
    pub demographic_quotas: DemographicQuotas,
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeGovernmentArgs {
    pub name: String,
    pub description: String,
    pub assembly_size: u8,
    pub regions: [u32; 10],
    pub age_groups: [u32; 5],
    pub other_demographic: [u32; 3],
    pub nft_config: Option<GovernanceTokenConfig>,
    pub spl_config: Option<GovernanceTokenConfig>,
    pub nft_symbol: String,
    pub spl_symbol: String,
    pub nft_supply: u64,
    pub spl_supply: u64,
    pub collection_price: u64,
    pub primary_governance_token: PrimaryGovernanceToken,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct GovernanceTokenConfig {
    pub token_type: GovernanceTokenType,
    pub token_mint: Pubkey,
}

#[derive(Default)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum GovernanceTokenType {
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct DemographicQuotas {
    pub regions: [u32; 10],
    pub age_groups: [u32; 5],
    pub other_demographic: [u32; 3],
}

impl GovernancePool {
    pub const SPL_PREFIX_SEED: &'static [u8] = b"spl";
    pub const NFT_PREFIX_SEED: &'static [u8] = b"nft";
    pub const SBT_PREFIX_SEED: &'static [u8] = b"sbt";
    pub const MAX_SYMBOL_LENGTH: usize = 10; // Assuming a maximum length for symbols

    pub const SPACE: usize = 8 +  // discriminator
        32 +  // admin: Pubkey
        4 +   // total_citizens: u32
        1 +   // assembly_size: u8
        8 +   // assembly_term: i64
        32 +  // current_assembly: Pubkey
        32 +  // last_random_seed: [u8; 32]
        1 +   // selection_in_progress: bool
        4 +   // total_citizen_indices: u32
        (4 * 10) + (4 * 5) + (4 * 3) +  // demographic_quotas: DemographicQuotas
        4 + Self::MAX_SYMBOL_LENGTH +  // nft_symbol: String
        4 + Self::MAX_SYMBOL_LENGTH +  // spl_symbol: String
        8 +   // collection_price: u64
        8 +   // resources: u64
        4 +   // nft_minted: u32
        8 +   // total_nft_token_supply: u64
        4 +   // spl_minted: u32
        8 +   // total_spl_token_supply: u64
        4 +   // sbt_minted: u32
        8 +   // total_sbt_token_supply: u64
        33 +  // spl_mint: Option<Pubkey> (1 for discriminator + 32 for Pubkey)
        33 +  // nft_mint: Option<Pubkey>
        33 +  // sbt_mint: Option<Pubkey>
        32 +  // governance_token_mint: Pubkey
        1;    // bump: u8
}

impl DemographicQuotas {
    pub fn reset(&mut self) {
        self.regions = [0; 10];
        self.age_groups = [0; 5];
        self.other_demographic = [0; 3];
    }
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