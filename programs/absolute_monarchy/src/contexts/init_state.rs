use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::states::{Monarch, Kingdom, InitializeKingdomArgs, KingdomTokenType, PrimaryKingdomToken};
use crate::error::AbsoluteMonarchyError;
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct InitializeKingdom<'info> {
    #[account(
        init, 
        payer = authority, 
        space = Kingdom::SPACE,
        seeds = [b"kingdom", monarch.key().as_ref()],
        bump
    )]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        init, 
        payer = authority, 
        space = Monarch::SPACE,
        seeds = [b"monarch", kingdom.key().as_ref()],
        bump
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: This account is optional and will be validated if provided
    #[account(mut)]
    pub nft_mint: Option<Account<'info, Mint>>,

    /// CHECK: This account is optional and will be validated if provided
    #[account(mut)]
    pub spl_mint: Option<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_kingdom(
    ctx: Context<InitializeKingdom>,
    args: InitializeKingdomArgs
) -> Result<()> {

    let kingdom = &mut ctx.accounts.kingdom;
    let monarch = &mut ctx.accounts.monarch;

    monarch.authority = *ctx.accounts.authority.key;
    monarch.name = args.monarch_name.clone();
    monarch.divine_mandate = args.divine_mandate;
    monarch.coronation_date = Clock::get()?.unix_timestamp;
    monarch.abdication_date = None;
    monarch.decrees_issued = 0;
    monarch.wars_declared = 0;
    monarch.royal_judgments = 0;
    monarch.economic_policies_set = 0;
    monarch.pardons_granted = 0;
    monarch.policies_implemented = 0;
    monarch.bump = ctx.bumps.monarch; 

    // pub wars_declared: u64,
    // pub royal_judgments: u64,
    // pub economic_policies_set: u64,
    // pub pardons_granted: u64,
    // pub policies_implemented: u64,

    kingdom.name = args.name;
    kingdom.description = args.description;
    kingdom.creator = ctx.accounts.monarch.key();
    kingdom.monarch = ctx.accounts.monarch.key();
    kingdom.monarch_name = args.monarch_name.clone();
    kingdom.total_decrees = 0;
    kingdom.total_active_decrees = 0;
    kingdom.total_subjects = 0;
    kingdom.min_loyalty_amount = args.min_loyalty_amount;
    kingdom.established_at = Clock::get()?.unix_timestamp;
    kingdom.pardons_granted = 0;
    kingdom.wars_declared = 0;
    kingdom.royal_judgments = 0;
    kingdom.economic_policies_set = 0;
    kingdom.policies_implemented = 0;
    kingdom.divisions = Vec::new();
    kingdom.officials_appointed = Vec::new();
    kingdom.nobles = Vec::new();
    kingdom.membership_tokens_threshold = args.membership_tokens_threshold;
    kingdom.bump = ctx.bumps.kingdom;

    if let Some(ref nft_config) = args.nft_config {
        match nft_config.token_type {
            KingdomTokenType::New => {
                require!(ctx.accounts.nft_mint.is_some(), AbsoluteMonarchyError::MissingRequiredAccount);
                let nft_mint = ctx.accounts.nft_mint.as_ref().unwrap();
                
                // Derive the PDA for the new NFT mint
                let (nft_mint_pda, _bump) = Pubkey::find_program_address(
                    &[Kingdom::NFT_PREFIX_SEED, kingdom.key().as_ref(), kingdom.nft_symbol.as_bytes()],
                    ctx.program_id
                );
                
                // Verify that the derived PDA matches the provided account
                require!(nft_mint.key() == nft_mint_pda, AbsoluteMonarchyError::InvalidMint);

                kingdom.nft_mint = Some(nft_mint.key());
                kingdom.total_nft_token_supply = nft_mint.supply;
            },
            KingdomTokenType::Existing => {
                require!(ctx.accounts.nft_mint.is_some(), AbsoluteMonarchyError::MissingRequiredAccount);
                let nft_mint = ctx.accounts.nft_mint.as_ref().unwrap();
                require!(nft_mint.key() == nft_config.custom_mint, AbsoluteMonarchyError::InvalidMint);
                kingdom.nft_mint = Some(nft_mint.key());
                kingdom.total_nft_token_supply = nft_mint.supply;
                kingdom.nft_minted = nft_mint.supply;
            },
        }
    }

    if let Some(ref spl_config) = args.spl_config {
        match spl_config.token_type {
            KingdomTokenType::New => {
                require!(ctx.accounts.spl_mint.is_some(), AbsoluteMonarchyError::MissingRequiredAccount);
                let spl_mint = ctx.accounts.spl_mint.as_ref().unwrap();
                // Derive the PDA for the new NFT mint
                let (spl_mint_pda, _bump) = Pubkey::find_program_address(
                    &[Kingdom::SPL_PREFIX_SEED, kingdom.key().as_ref(), kingdom.spl_symbol.as_bytes()],
                    ctx.program_id
                );
                
                // Verify that the derived PDA matches the provided account
                require!(spl_mint.key() == spl_mint_pda, AbsoluteMonarchyError::InvalidMint);
                kingdom.spl_mint = Some(spl_mint.key());
                kingdom.total_spl_token_supply = spl_mint.supply;
            },
            KingdomTokenType::Existing => {
                require!(ctx.accounts.spl_mint.is_some(), AbsoluteMonarchyError::MissingRequiredAccount);
                let existing_spl_mint = ctx.accounts.spl_mint.as_ref().unwrap();
                require!(existing_spl_mint.key() == spl_config.custom_mint, AbsoluteMonarchyError::InvalidMint);
                kingdom.spl_mint = Some(existing_spl_mint.key());
                kingdom.total_spl_token_supply = existing_spl_mint.supply;
                kingdom.spl_minted = existing_spl_mint.supply;
            },
        }
    }

    match args.primary_kingdom_token {
        PrimaryKingdomToken::NFT => {
            kingdom.primary_token_mint = kingdom.nft_mint.unwrap();
        },
        PrimaryKingdomToken::SPL => {
            kingdom.primary_token_mint = kingdom.spl_mint.unwrap();
        },
    }

    require!(args.nft_config.is_some() || args.spl_config.is_some(), AbsoluteMonarchyError::NoKingdomTokenSpecified);



    Ok(())
}
