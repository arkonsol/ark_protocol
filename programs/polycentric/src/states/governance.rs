use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::GovernanceError;

#[account]
pub struct GovernancePool {
    pub name: String,
    pub description: String,
    pub admin: Pubkey,
    pub assemblies: Vec<Pubkey>,
    pub policy_areas: Vec<Pubkey>,
    pub treasuries: Vec<Pubkey>,
    pub total_participants: u64,
    pub total_proposals: u64,
    pub total_votes: u64,
    pub tasks: Vec<Pubkey>,
    pub total_tasks: u64,
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
    pub membership_token_threshold: u64,
    pub bump: u8,

}

impl GovernancePool {

    pub const MAX_NAME_LENGTH: usize = 50;  
    pub const MAX_DESCRIPTION_LENGTH: usize = 150;  
    pub const MAX_ASSEMBLIES: usize = 5;  
    pub const MAX_POLICY_AREAS: usize = 5;  
    pub const MAX_TASKS: usize = 10;  
    pub const MAX_TREASURIES: usize = 5;
    pub const SPL_PREFIX_SEED: &'static [u8] = b"spl";
    pub const NFT_PREFIX_SEED: &'static [u8] = b"nft";

    pub const SPACE: usize = 8 +  // discriminator
        4 + Self::MAX_NAME_LENGTH +  // name
        4 + Self::MAX_DESCRIPTION_LENGTH +  // description
        32 +  // admin
        4 + (32 * Self::MAX_ASSEMBLIES) +  // assemblies
        4 + (32 * Self::MAX_POLICY_AREAS) +  // policy_areas
        4 + (32 * Self::MAX_TREASURIES) +  // treasuries
        8 +  // total_participants
        8 +  // total_proposals
        8 +  // total_votes
        4 + (32 * Self::MAX_TASKS) +  // tasks
        8 +  // total_tasks
        4 + Self::MAX_NAME_LENGTH +  // nft_symbol
        4 + Self::MAX_NAME_LENGTH +  // spl_symbol
        8 +  // collection_price
        8 +  // resources
        4 +  // nft_minted
        8 +  // total_nft_token_supply
        4 +  // spl_minted
        8 +  // total_spl_token_supply
        4 +  // sbt_minted
        8 +  // total_sbt_token_supply
        32 +  // spl_mint
        32 +  // nft_mint
        32 +  // sbt_mint
        32 +  // governance_token_mint
        8 +  // membership_token_threshold
        1;   // bump


    pub fn add_assembly(&mut self, assembly: Pubkey) -> Result<()> {
        require!(self.assemblies.len() < MAX_ASSEMBLIES, GovernanceError::MaxAssembliesReached);
        self.assemblies.push(assembly);
        Ok(())
    }

    pub fn add_policy_area(&mut self, policy_area: Pubkey) -> Result<()> {
        require!(self.policy_areas.len() < MAX_POLICY_AREAS, GovernanceError::MaxPolicyAreasReached);
        self.policy_areas.push(policy_area);
        Ok(())
    }

    pub fn increment_participants(&mut self) {
        self.total_participants += 1;
    }

    pub fn increment_proposals(&mut self) {
        self.total_proposals += 1;
    }

    pub fn increment_votes(&mut self) {
        self.total_votes += 1;
    }

    pub fn add_treasury(&mut self, treasury: Pubkey) -> Result<()> {
        require!(self.treasuries.len() < Self::MAX_TREASURIES, GovernanceError::MaxTreasuriesReached);
        self.treasuries.push(treasury);
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeGovernmentArgs {
    pub name: String,
    pub description: String,
    pub nft_config: Option<GovernanceTokenConfig>,
    pub spl_config: Option<GovernanceTokenConfig>,
    pub primary_governance_token: PrimaryGovernanceToken,
    pub nft_symbol: String,
    pub spl_symbol: String,
    pub collection_price: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum GovernanceTokenType {
    New,
    Existing,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GovernanceTokenConfig {
    pub token_type: GovernanceTokenType,
    pub token_mint: Pubkey,
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