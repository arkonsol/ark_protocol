use anchor_lang::prelude::*;
use crate::{states::junta::Junta, InitializeJuntaArgs, JuntaTokenType, PrimaryJuntaToken};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token},
};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(args: InitializeJuntaArgs)]
pub struct InitializeJunta<'info> {
    #[account(
        init,
        payer = leader,
        space = Junta::SPACE,
        seeds = [Junta::PREFIX_SEED, args.name.as_bytes()], 
        bump
    )]
    pub junta: Account<'info, Junta>,

    #[account(mut)]
    pub leader: Signer<'info>,

    /// CHECK: This account is optional and will be validated if provided
    #[account(mut)]
    pub nft_mint: Option<Account<'info, Mint>>,

    /// CHECK: This account is optional and will be validated if provided
    #[account(mut)]
    pub spl_mint: Option<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_junta(ctx: Context<InitializeJunta>, args: InitializeJuntaArgs) -> Result<()> {
    let junta = &mut ctx.accounts.junta;

    junta.name = args.name;
    junta.leader = ctx.accounts.leader.key();
    junta.officers = vec![];
    junta.resources = 100;
    junta.decrees = vec![];
    junta.dissent_level = 0;
    junta.control_level = 200;
    junta.martial_law_active = false;
    junta.is_overthrown = false;
    junta.support_threshold = args.support_threshold;
    junta.bump = ctx.bumps.junta;

    if let Some(ref spl_config) = args.spl_config {
        require!(ctx.accounts.spl_mint.is_some(), ErrorCode::MissingRequiredAccount);
        let spl_mint = ctx.accounts.spl_mint.as_ref().unwrap();

        match spl_config.token_type {
            JuntaTokenType::New => {
                let (expected_mint_pda, _) = Pubkey::find_program_address(
                    &[Junta::SPL_PREFIX_SEED, junta.key().as_ref(), args.spl_symbol.as_bytes()],
                    ctx.program_id
                );
                require!(spl_mint.key() == expected_mint_pda, ErrorCode::InvalidMint);
            },
            JuntaTokenType::Existing => {
                require!(spl_mint.key() == spl_config.token_mint, ErrorCode::InvalidMint);
            },
        }
        junta.spl_mint = Some(spl_mint.key());
        junta.total_spl_token_supply = spl_mint.supply;
        junta.spl_minted = 0;
        junta.spl_symbol = args.spl_symbol;
    }

    // Handle NFT configuration
    if let Some(ref nft_config) = args.nft_config {
        require!(ctx.accounts.nft_mint.is_some(), ErrorCode::MissingRequiredAccount);
        let nft_mint = ctx.accounts.nft_mint.as_ref().unwrap();

        match nft_config.token_type {
            JuntaTokenType::New => {
                let (expected_mint_pda, _) = Pubkey::find_program_address(
                    &[Junta::NFT_PREFIX_SEED, junta.key().as_ref(), args.nft_symbol.as_bytes()],
                    ctx.program_id
                );
                require!(nft_mint.key() == expected_mint_pda, ErrorCode::InvalidMint);
            },
            JuntaTokenType::Existing => {
                require!(nft_mint.key() == nft_config.token_mint, ErrorCode::InvalidMint);
            },
        }
        junta.nft_mint = Some(nft_mint.key());
        junta.total_nft_token_supply = nft_mint.supply;
        junta.nft_minted = 0;
        junta.nft_symbol = args.nft_symbol;
        junta.collection_price = args.collection_price;
    }

    // Set primary governance token
    match args.primary_junta_token {
        PrimaryJuntaToken::NFT => {
            require!(args.nft_config.is_some(), ErrorCode::MissingNFTConfig);
            junta.governance_token_mint = junta.nft_mint.unwrap();
        },
        PrimaryJuntaToken::SPL => {
            require!(args.spl_config.is_some(), ErrorCode::MissingSPLConfig);
            junta.governance_token_mint = junta.spl_mint.unwrap();
        },
    }

    require!(args.nft_config.is_some() || args.spl_config.is_some(), ErrorCode::NoJuntaTokenSpecified);

    Ok(())
}