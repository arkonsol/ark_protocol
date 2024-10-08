use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::states::{Governance, InitializeGovernanceArgs, GovernanceTokenType, PrimaryGovernanceToken};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct InitializeGovernance<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = Governance::SPACE,
        seeds = [b"governance", governance.key().as_ref()],
        bump
    )]
    pub governance: Box<Account<'info, Governance>>,

    /// CHECK: This account is optional and will be validated if provided
    #[account(mut)]
    pub nft_mint: Option<Account<'info, Mint>>,

    /// CHECK: This account is optional and will be validated if provided
    #[account(mut)]
    pub spl_mint: Option<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn initialize_governance(
    ctx: Context<InitializeGovernance>,
    args: InitializeGovernanceArgs
) -> Result<()> {

    // pub name: String,
    // pub description: String,
    // pub authority: Pubkey,
    // pub governance_token_mint: Pubkey,
    // pub approval_threshold: u64,
    // pub total_proposals: u64,
    // pub total_members: u64,
    // pub min_stake_amount: u64,
    // pub max_lock_period: u8,
    // pub voting_delay: i64,
    // pub voting_period: i64,

    let nft_config = args.nft_config.clone();
    let spl_config = args.spl_config.clone();
    let primary_governance_token = args.primary_governance_token;

    let governance = &mut ctx.accounts.governance;
    governance.name = args.name;
    governance.description = args.description;
    governance.creator = ctx.accounts.authority.key();
    governance.total_proposals = 0;
    governance.total_members = 0;
    governance.min_stake_amount = args.min_stake_amount;
    governance.approval_threshold = args.approval_threshold;
    governance.collection_price = args.collection_price;
    governance.bump = ctx.bumps.governance;

    if let Some(ref nft_config) = nft_config {
        match nft_config.token_type {
            GovernanceTokenType::New => {
                require!(ctx.accounts.nft_mint.is_some(), ErrorCode::MissingRequiredAccount);
                let nft_mint = ctx.accounts.nft_mint.as_ref().unwrap();
                // Derive the expected PDA for the NFT mint
                let (nft_mint_pda, _bump) = Pubkey::find_program_address(
                    &[
                        Governance::NFT_PREFIX_SEED,
                        governance.key().as_ref(),
                        args.nft_symbol.as_bytes(),
                    ],
                    ctx.program_id
                );

                require!(nft_mint.key() == nft_mint_pda, ErrorCode::InvalidMint);
                
                // Store the derived PDA in the governance state
                governance.nft_mint = Some(nft_mint_pda);
                governance.total_nft_token_supply = nft_mint.supply;
            },
            GovernanceTokenType::Existing => {
                require!(ctx.accounts.nft_mint.is_some(), ErrorCode::MissingRequiredAccount);
                let nft_mint = ctx.accounts.nft_mint.as_ref().unwrap();
                require!(nft_mint.key() == nft_config.custom_mint, ErrorCode::InvalidMint);
                governance.nft_mint = Some(nft_mint.key());
                governance.total_nft_token_supply = nft_mint.supply;
                governance.nft_minted = nft_mint.supply as u32;
            },
        }
    }

    if let Some(ref spl_config) = spl_config {
        match spl_config.token_type {
            GovernanceTokenType::New => {
                require!(ctx.accounts.spl_mint.is_some(), ErrorCode::MissingRequiredAccount);
                let spl_mint = ctx.accounts.spl_mint.as_ref().unwrap();
                
                // Derive the expected PDA for the SPL token mint
                let (expected_mint_pda, _bump) = Pubkey::find_program_address(
                    &[
                        Governance::SPL_PREFIX_SEED,
                        governance.key().as_ref(),
                        governance.spl_symbol.as_bytes(),
                    ],
                    ctx.program_id
                );
                
                // Verify that the provided new_spl_mint matches the expected PDA
                require!(spl_mint.key() == expected_mint_pda, ErrorCode::InvalidSPLMint);

                governance.spl_mint = Some(spl_mint.key());
                governance.total_spl_token_supply = spl_mint.supply;

            },
            GovernanceTokenType::Existing => {
                require!(ctx.accounts.spl_mint.is_some(), ErrorCode::MissingRequiredAccount);
                let spl_mint = ctx.accounts.spl_mint.as_ref().unwrap();
                require!(spl_mint.key() == spl_config.custom_mint, ErrorCode::InvalidSPLMint);
                governance.spl_mint = Some(spl_mint.key());
                governance.total_spl_token_supply = spl_mint.supply;
                governance.spl_minted = spl_mint.supply as u32;
            },
        }
    }

    match primary_governance_token {
        PrimaryGovernanceToken::NFT => {
            match nft_config {
                Some(_) => {
                    governance.governance_token_mint = governance.nft_mint.unwrap();
                },
                None => return Err(ErrorCode::MissingNFTConfig.into()),
            }
        },
        PrimaryGovernanceToken::SPL => {
            match spl_config {
                Some(_) => {
                    governance.governance_token_mint = governance.spl_mint.unwrap();
                },
                None => return Err(ErrorCode::MissingSPLConfig.into()),
            }
        },
    }

    // Ensure at least one governance token is specified
    require!(nft_config.is_some() || spl_config.is_some(), ErrorCode::NoGovernanceTokenSpecified);

    Ok(())
}


