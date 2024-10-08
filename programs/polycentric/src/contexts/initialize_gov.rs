use anchor_lang::prelude::*;
use crate::states::{GovernancePool, Analytics, InitializeGovernmentArgs, GovernanceTokenType, PrimaryGovernanceToken};
use crate::error::GovernanceError;
use crate::constants::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction(args: InitializeGovernmentArgs)]
pub struct InitializeGovernment<'info> {
    #[account(
        init,
        payer = admin,
        space = GovernancePool::SPACE,
        seeds = [b"governance_pool", admin.key().as_ref()],
        bump
    )]
    pub governance_pool: Box<Account<'info, GovernancePool>>,

    #[account(mut)]
    mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(mut)]
    pub analytics: Box<Account<'info, Analytics>>,
    
    /// CHECK: This account is optional and will be validated if provided
    #[account(mut)]
    pub nft_mint: Option<Account<'info, Mint>>,
    
    /// CHECK: This account is optional and will be validated if provided
    #[account(mut)]
    pub spl_mint: Option<Account<'info, Mint>>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<InitializeGovernment>, args: InitializeGovernmentArgs) -> Result<()> {
    require!(args.name.len() <= MAX_NAME_LENGTH, GovernanceError::InvalidGovernancePool);
    require!(args.description.len() <= MAX_DESCRIPTION_LENGTH, GovernanceError::InvalidGovernancePool);

    let governance_pool = &mut ctx.accounts.governance_pool;
    let analytics = &mut ctx.accounts.analytics;
    let clock = &ctx.accounts.clock;

    governance_pool.name = args.name;
    governance_pool.description = args.description;
    governance_pool.admin = ctx.accounts.admin.key();
    governance_pool.assemblies = Vec::new();
    governance_pool.policy_areas = Vec::new();
    governance_pool.treasuries = Vec::new();
    governance_pool.total_participants = 0;
    governance_pool.total_proposals = 0;
    governance_pool.total_votes = 0;
    governance_pool.tasks = Vec::new();
    governance_pool.total_tasks = 0;
    governance_pool.nft_symbol = args.nft_symbol.clone();
    governance_pool.spl_symbol = args.spl_symbol.clone();
    governance_pool.collection_price = args.collection_price;
    governance_pool.resources = 0;
    governance_pool.bump = ctx.bumps.governance_pool;


    if let Some(ref nft_config) = args.nft_config {
        match nft_config.token_type {
            GovernanceTokenType::New => {
                require!(ctx.accounts.nft_mint.is_some(), GovernanceError::MissingRequiredAccount);
                let nft_mint = ctx.accounts.nft_mint.as_ref().unwrap();
                let (nft_mint_pda, _bump) = Pubkey::find_program_address(
                    &[
                        GovernancePool::NFT_PREFIX_SEED,
                        governance_pool.key().as_ref(),
                        args.nft_symbol.as_bytes(),
                    ],
                    ctx.program_id
                );
                require!(nft_mint.key() == nft_mint_pda, GovernanceError::InvalidNFTMint);
                governance_pool.nft_mint = Some(nft_mint_pda);
                governance_pool.total_nft_token_supply = nft_mint.supply;
            },
            GovernanceTokenType::Existing => {
                require!(ctx.accounts.nft_mint.is_some(), GovernanceError::MissingRequiredAccount);
                let nft_mint = ctx.accounts.nft_mint.as_ref().unwrap();
                require!(nft_mint.key() == nft_config.token_mint, GovernanceError::InvalidNFTMint);
                governance_pool.nft_mint = Some(nft_mint.key());
                governance_pool.total_nft_token_supply = nft_mint.supply;
            },
        }
    }

    if let Some(ref spl_config) = args.spl_config {
        match spl_config.token_type {
            GovernanceTokenType::New => {
                require!(ctx.accounts.spl_mint.is_some(), GovernanceError::MissingRequiredAccount);
                let spl_mint = ctx.accounts.spl_mint.as_ref().unwrap();
                let (expected_mint_pda, _bump) = Pubkey::find_program_address(
                    &[
                        GovernancePool::SPL_PREFIX_SEED,
                        governance_pool.key().as_ref(),
                        governance_pool.spl_symbol.as_bytes(),
                    ],
                    ctx.program_id
                );
                require!(spl_mint.key() == expected_mint_pda, GovernanceError::InvalidSPLMint);
                governance_pool.spl_mint = Some(spl_mint.key());
                governance_pool.total_spl_token_supply = spl_mint.supply;
            },
            GovernanceTokenType::Existing => {
                require!(ctx.accounts.spl_mint.is_some(), GovernanceError::MissingRequiredAccount);
                let spl_mint = ctx.accounts.spl_mint.as_ref().unwrap();
                require!(spl_mint.key() == spl_config.token_mint, GovernanceError::InvalidSPLMint);
                governance_pool.spl_mint = Some(spl_mint.key());
                governance_pool.total_spl_token_supply = spl_mint.supply;
            },
        }
    }

    match args.primary_governance_token {
        PrimaryGovernanceToken::NFT => {
            match args.nft_config {
                Some(_) => {
                    governance_pool.governance_token_mint = governance_pool.nft_mint.unwrap();
                },
                None => return Err(GovernanceError::MissingNFTConfig.into()),
            }
        },
        PrimaryGovernanceToken::SPL => {
            match args.spl_config {
                Some(_) => {
                    governance_pool.governance_token_mint = governance_pool.spl_mint.unwrap();
                },
                None => return Err(GovernanceError::MissingSPLConfig.into()),
            }
        },
    }

    require!(args.nft_config.is_some() || args.spl_config.is_some(), GovernanceError::NoGovernanceTokenSpecified);

    // Update analytics
    analytics.increment_governance_pools();
    analytics.update_timestamp(clock.unix_timestamp);

    Ok(())
}