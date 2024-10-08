use anchor_lang::prelude::*;
use crate::contexts::InitTokenParams;
use crate::states::{InitializeSbtArgs, MintNftArgs};

#[account]
pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub creator: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub execution_time: i64,
    pub for_votes: u64,
    pub against_votes: u64,
    pub status: ProposalStatus,
    pub param_name: Option<String>,
    pub param_value: Option<u64>,
    pub transfer_amount: Option<u64>,
    pub proposal_type: ProposalType,
    pub mint_amount_treasury: Option<u64>,
    pub mint_amount_citizen: Option<u64>,
    pub init_token_params: Option<InitTokenParams>,
    pub nft_args: Option<MintNftArgs>,
    pub sbt_args: Option<InitializeSbtArgs>,
    pub total_staked: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalType {
    InitializeToken,
    MintTokens,
    Transfer,
    UpdateParameter,
    MintNft,
    MintSbt,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

impl Proposal {
    pub const MAX_DESCRIPTION_LEN: usize = 128;
    pub const MAX_PARAM_NAME_LEN: usize = 32;
    pub const MAX_TOKEN_NAME_LEN: usize = 32;
    pub const MAX_TOKEN_SYMBOL_LEN: usize = 10;
    pub const MAX_URI_LEN: usize = 200;

    pub const SPACE: usize = 8 +  // discriminator
        8 +  // id
        4 + Self::MAX_DESCRIPTION_LEN +  // description (4 bytes for length + content)
        32 +  // creator (Pubkey)
        8 +   // start_time
        8 +   // end_time
        8 +   // execution_time
        8 +   // for_votes
        8 +   // against_votes
        1 +   // status (enum)
        1 + 4 + Self::MAX_PARAM_NAME_LEN +  // param_name (optional string: 1 byte + 4 length + content)
        1 + 8 +  // param_value (optional u64: 1 byte + 8 bytes)
        1 + 8 +  // transfer_amount (optional u64: 1 byte + 8 bytes)
        1 +   // proposal_type (enum)
        1 + 8 +  // mint_amount_treasury (optional u64: 1 byte + 8 bytes)
        1 + 8 +  // mint_amount_citizen (optional u64: 1 byte + 8 bytes)
        1 + (
            4 + Self::MAX_TOKEN_NAME_LEN +
            4 + Self::MAX_TOKEN_SYMBOL_LEN +
            4 + Self::MAX_URI_LEN +
            1  // decimals (u8)
        ) +  // init_token_params (optional)
        1 + (
            4 + Self::MAX_TOKEN_NAME_LEN +
            4 + Self::MAX_TOKEN_SYMBOL_LEN +
            4 + Self::MAX_URI_LEN
        ) +  // nft_args (optional)
        1 + (
            4 + Self::MAX_TOKEN_NAME_LEN +
            4 + // supply (u32)
            4 + Self::MAX_TOKEN_SYMBOL_LEN +
            1 + // transferrable (bool)
            4 + Self::MAX_URI_LEN
        ) +  // sbt_args (optional)
        8 + // total_staked
        1;  // bump

    // ... other impl methods ...
}