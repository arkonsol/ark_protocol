use crate::error::AbsoluteMonarchyError;
use crate::states::kingdom::Kingdom;
use crate::states::subject::Subject;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount, initialize_mint, InitializeMint},
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, 
        Metadata as Metaplex,
    },
};

#[derive(Accounts)]
#[instruction(params: InitTokenParams)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        mint::decimals = params.decimals,
        mint::authority = mint,
        seeds = [Kingdom::SPL_PREFIX_SEED, kingdom.key().as_ref(), params.symbol.as_bytes()], 
        bump
    )]
    pub mint: Account<'info, Mint>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metaplex>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintToKingdom<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = kingdom,
    )]
    pub kingdom_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintToSubject<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        space = Subject::SPACE,
        seeds = [b"subject", kingdom.key().as_ref(), &kingdom.total_subjects.to_le_bytes()],
        bump
    )]
    pub subject: Box<Account<'info, Subject>>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = subject
    )]
    pub subject_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}

pub fn initialize_token(
    ctx: Context<InitializeToken>,
    params: InitTokenParams
) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;
    let kingdom_key = kingdom.key();
    let payer = &ctx.accounts.payer.key();
    kingdom.spl_symbol = params.symbol.clone();

    let metadata_seeds = &[
        Kingdom::SPL_PREFIX_SEED,
        kingdom_key.as_ref(),
        params.symbol.as_bytes(),
        &[ctx.bumps.mint]
    ];
    let signer = &[&metadata_seeds[..]];

    // Initialize SPL Token Mint
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    );
    initialize_mint(cpi_context, params.decimals, payer, Some(&kingdom_key))?;

    let token_data = DataV2 {
        name: params.name.clone(),
        symbol: params.symbol.clone(),
        uri: params.uri,
        seller_fee_basis_points: 500,
        creators: None,
        collection: None,
        uses: None,
    };

    let metadata_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.mint.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            mint_authority: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer
    );

    create_metadata_accounts_v3(
        metadata_ctx,
        token_data,
        false,
        true,
        None,
    )?;

    kingdom.spl_mint = Some(ctx.accounts.mint.key());

    msg!("Token mint created successfully.");

    Ok(())
}

pub fn mint_to_kingdom(ctx: Context<MintToKingdom>, amount: u64) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;

    // Check if the new total minted would exceed supply
    if kingdom.spl_minted as u64 + amount > kingdom.total_spl_token_supply as u64 {
        return Err(AbsoluteMonarchyError::ExceedsSupply.into());
    }

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.kingdom_ata.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    mint_to(cpi_ctx, amount)?;

    // Update kingdom state
    kingdom.spl_minted = kingdom.spl_minted.checked_add(amount)
        .ok_or(AbsoluteMonarchyError::Overflow)?;

    // Emit an event
    emit!(MintEvent {
        kingdom: kingdom.key(),
        treasury_amount: amount,
        subject_amount: 0,
        total_minted: kingdom.spl_minted,
    });

    Ok(())
}

pub fn mint_to_subject(ctx: Context<MintToSubject>, amount: u64) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;

    // Check if the new total minted would exceed supply
    if kingdom.spl_minted as u64 + amount > kingdom.total_spl_token_supply as u64 {
        return Err(AbsoluteMonarchyError::ExceedsSupply.into());
    }

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.subject_token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    mint_to(cpi_ctx, amount)?;

    // Update kingdom state
    kingdom.spl_minted = kingdom.spl_minted.checked_add(amount)
        .ok_or(AbsoluteMonarchyError::Overflow)?;

    // Update subject information
    let subject = &mut ctx.accounts.subject;
    subject.key = subject.key();
    subject.role = "Citizen".to_string();
    subject.jurisdiction = "Kingdom".to_string();
    subject.loyalty = 50; // Start with neutral loyalty
    subject.wealth = amount;
    subject.is_convicted = false;
    subject.appointed_at = Clock::get()?.unix_timestamp;

    kingdom.total_subjects += 1;

    // Emit an event
    emit!(MintEvent {
        kingdom: kingdom.key(),
        treasury_amount: 0,
        subject_amount: amount,
        total_minted: kingdom.spl_minted,
    });

    Ok(())
}

#[event]
pub struct MintEvent {
    pub kingdom: Pubkey,
    pub treasury_amount: u64,
    pub subject_amount: u64,
    pub total_minted: u64,
}