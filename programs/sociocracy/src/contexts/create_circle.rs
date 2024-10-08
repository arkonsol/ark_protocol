use anchor_lang::prelude::*;
use crate::states::{Circle, CircleTokenType, CreateCircleArgs, PrimaryGovernanceToken};
use anchor_spl::token::Mint;
use crate::errors::GovernanceError;

#[derive(Accounts)]
#[instruction(args: CreateCircleArgs)]
pub struct CreateCircle<'info> {
    #[account(
        init,
        payer = payer,
        space = Circle::SPACE,
        seeds = [b"circle", args.name.as_bytes()],
        bump
    )]
    pub circle: Box<Account<'info, Circle>>,

    #[account(mut)]
    pub payer: Signer<'info>,

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

pub fn create_circle(ctx: Context<CreateCircle>, args: CreateCircleArgs) -> Result<()> {
    require!(args.name.len() <= Circle::MAX_NAME_LENGTH, GovernanceError::NameTooLong);
    require!(args.description.len() <= Circle::MAX_DESCRIPTION_LENGTH, GovernanceError::DescriptionTooLong);

    let circle = &mut ctx.accounts.circle;
    let clock = &ctx.accounts.clock;

    circle.name = args.name;
    circle.description = args.description;
    circle.circle_type = args.circle_type;
    circle.created_at = clock.unix_timestamp;
    circle.updated_at = clock.unix_timestamp;
    circle.members = vec![];
    circle.proposals = vec![];
    circle.bump = ctx.bumps.circle;

    if args.spl_config.is_some() {
        require!(ctx.accounts.spl_mint.is_some(), GovernanceError::MissingAccount);
        let spl_mint = ctx.accounts.spl_mint.as_ref().unwrap();

        if let Some(ref spl_config) = args.spl_config {
            match spl_config.token_type {
                CircleTokenType::New => {
                    let (expected_mint_pda, _bump) = Pubkey::find_program_address(
                        &[
                            Circle::SPL_PREFIX_SEED,
                            circle.key().as_ref(),
                            circle.name.as_bytes(),
                        ],
                        ctx.program_id
                    );
                    require!(spl_mint.key() == expected_mint_pda, GovernanceError::InvalidSPLMint);
                },
                CircleTokenType::Existing => {
                    require!(spl_mint.key() == spl_config.token_mint, GovernanceError::InvalidSPLMint);
                },
            }
        }
        circle.spl_mint = Some(spl_mint.key());
        circle.total_spl_token_supply = spl_mint.supply;
    }

    if args.nft_config.is_some() {
        require!(ctx.accounts.nft_mint.is_some(), GovernanceError::MissingAccount);
        let nft_mint = ctx.accounts.nft_mint.as_ref().unwrap();

        if let Some(ref nft_config) = args.nft_config {
            match nft_config.token_type {
                CircleTokenType::New => {
                    let (expected_mint_pda, _bump) = Pubkey::find_program_address(
                        &[
                            Circle::NFT_PREFIX_SEED,
                            circle.key().as_ref(),
                            circle.name.as_bytes(),
                        ],
                        ctx.program_id
                    );
                    require!(nft_mint.key() == expected_mint_pda, GovernanceError::InvalidNFTMint);
                },
                CircleTokenType::Existing => {
                    require!(nft_mint.key() == nft_config.token_mint, GovernanceError::InvalidNFTMint);
                },
            }
        }
        circle.nft_mint = Some(nft_mint.key());
        circle.total_nft_token_supply = nft_mint.supply;
    }

    match args.primary_governance_token {
        PrimaryGovernanceToken::NFT => {
            require!(args.nft_config.is_some(), GovernanceError::MissingNFTConfig);
            circle.governance_token_mint = circle.nft_mint.unwrap();
        },
        PrimaryGovernanceToken::SPL => {
            require!(args.spl_config.is_some(), GovernanceError::MissingSPLConfig);
            circle.governance_token_mint = circle.spl_mint.unwrap();
        },
    }

    require!(args.nft_config.is_some() || args.spl_config.is_some(), GovernanceError::NoGovernanceTokenSpecified);

    // Add the circle creator as the first member
    circle.members.push(ctx.accounts.payer.key());

    Ok(())
}