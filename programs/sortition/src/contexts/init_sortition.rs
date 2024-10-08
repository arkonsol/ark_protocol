use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::states::{GovernancePool, DemographicQuotas, InitializeGovernmentArgs, GovernanceTokenType, PrimaryGovernanceToken};
use crate::error::GovernanceError;

#[derive(Accounts)]
#[instruction(args: InitializeGovernmentArgs)]
pub struct InitializeGovernance<'info> {
    #[account(
        init, 
        payer = admin, 
        space = GovernancePool::SPACE,
        seeds = [b"governance_pool", admin.key().as_ref()],
        bump
    )]
    pub governance_pool: Account<'info, GovernancePool>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: This account is optional and will be validated if provided
    #[account(mut)]
    pub nft_mint: Option<Account<'info, Mint>>,

    /// CHECK: This account is optional and will be validated if provided
    #[account(mut)]
    pub spl_mint: Option<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_governance(ctx: Context<InitializeGovernance>, args: InitializeGovernmentArgs) -> Result<()> {
    require!(args.assembly_size > 0 && args.assembly_size <= 100, GovernanceError::InvalidAssemblySize);
    
    // Validate that quotas sum up to assembly_size
    let total_regions = args.regions.iter().sum::<u32>();
    let total_age_groups = args.age_groups.iter().sum::<u32>();
    let total_other_demographic = args.other_demographic.iter().sum::<u32>();

    let total_quota = total_regions.max(total_age_groups).max(total_other_demographic);
    require!(total_quota == args.assembly_size as u32, GovernanceError::InvalidQuotas);

    let governance_pool = &mut ctx.accounts.governance_pool;
    governance_pool.admin = ctx.accounts.admin.key();
    governance_pool.total_citizens = 0;
    governance_pool.assembly_size = args.assembly_size;
    governance_pool.assembly_term = 0;
    governance_pool.current_assembly = Pubkey::default();
    governance_pool.last_random_seed = [0; 32];
    governance_pool.selection_in_progress = false;
    governance_pool.total_citizen_indices = 0;
    governance_pool.demographic_quotas = DemographicQuotas {
        regions: args.regions,
        age_groups: args.age_groups,
        other_demographic: args.other_demographic,
    };

    // Handle NFT configuration
    if let Some(ref nft_config) = args.nft_config {
        require!(ctx.accounts.nft_mint.is_some(), GovernanceError::MissingRequiredAccount);
        let nft_mint = ctx.accounts.nft_mint.as_ref().unwrap();

        match nft_config.token_type {
            GovernanceTokenType::New => {
                let (expected_mint_pda, _) = Pubkey::find_program_address(
                    &[GovernancePool::NFT_PREFIX_SEED, governance_pool.key().as_ref(), args.nft_symbol.as_bytes()],
                    ctx.program_id
                );
                require!(nft_mint.key() == expected_mint_pda, GovernanceError::InvalidMint);
            },
            GovernanceTokenType::Existing => {
                require!(nft_mint.key() == nft_config.token_mint, GovernanceError::InvalidMint);
            },
        }
        governance_pool.nft_mint = Some(nft_mint.key());
        governance_pool.total_nft_token_supply = nft_mint.supply;
        governance_pool.nft_symbol = args.nft_symbol;
    }

    // Handle SPL token configuration
    if let Some(ref spl_config) = args.spl_config {
        require!(ctx.accounts.spl_mint.is_some(), GovernanceError::MissingRequiredAccount);
        let spl_mint = ctx.accounts.spl_mint.as_ref().unwrap();

        match spl_config.token_type {
            GovernanceTokenType::New => {
                let (expected_mint_pda, _) = Pubkey::find_program_address(
                    &[GovernancePool::SPL_PREFIX_SEED, governance_pool.key().as_ref(), args.spl_symbol.as_bytes()],
                    ctx.program_id
                );
                require!(spl_mint.key() == expected_mint_pda, GovernanceError::InvalidMint);
            },
            GovernanceTokenType::Existing => {
                require!(spl_mint.key() == spl_config.token_mint, GovernanceError::InvalidMint);
            },
        }
        governance_pool.spl_mint = Some(spl_mint.key());
        governance_pool.total_spl_token_supply = spl_mint.supply;
        governance_pool.spl_symbol = args.spl_symbol;
    }

    // Set primary governance token
    match args.primary_governance_token {
        PrimaryGovernanceToken::NFT => {
            require!(args.nft_config.is_some(), GovernanceError::MissingNFTConfig);
            governance_pool.governance_token_mint = governance_pool.nft_mint.unwrap();
        },
        PrimaryGovernanceToken::SPL => {
            require!(args.spl_config.is_some(), GovernanceError::MissingSPLConfig);
            governance_pool.governance_token_mint = governance_pool.spl_mint.unwrap();
        },
    }

    require!(args.nft_config.is_some() || args.spl_config.is_some(), GovernanceError::NoTokenSpecified);

    Ok(())
}