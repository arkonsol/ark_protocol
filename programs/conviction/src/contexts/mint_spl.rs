use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use crate::states::governance::Governance;
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
    pub governance: Box<Account<'info, Governance>>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        mint::decimals = params.decimals,
        mint::authority = mint,
        seeds = [Governance::SPL_PREFIX_SEED, governance.key().as_ref(), params.symbol.as_bytes()], 
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
pub struct MintTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub governance: Box<Account<'info, Governance>>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = governance,
    )]
    pub governance_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub citizen_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
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
    let governance = &mut ctx.accounts.governance;
    let governance_key = governance.key();
    let payer = &ctx.accounts.payer.key();
    governance.spl_symbol = params.symbol.clone();



    let metadata_seeds = &[
        Governance::SPL_PREFIX_SEED,
        governance_key.as_ref(),
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
    initialize_mint(cpi_context, params.decimals, &payer.key(), Some(&governance.key()))?;

    let token_data = DataV2 {
        name: params.name.clone(),
        symbol: params.symbol.clone(),
        uri: params.uri,
        seller_fee_basis_points: 500,
        creators: None, // creators list
        collection: None, // collection info
        uses: None, // use cases 
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

    msg!("Token mint created successfully.");

    Ok(())
}

pub fn mint_tokens(ctx: Context<MintTokens>, amount_to_treasury: u64, amount_to_citizen: u64) -> Result<()> {
    let governance = &mut ctx.accounts.governance;


    // Checking for overflow
    let total_mint_amount = amount_to_treasury.checked_add(amount_to_citizen)
        .ok_or(ErrorCode::Overflow)?;

    // Checking if the new total minted would exceed supply
    if governance.spl_minted as u64 + total_mint_amount > governance.total_spl_token_supply as u64 {
        return Err(ErrorCode::ExceedsSupply.into());
    }

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.governance_ata.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    mint_to(cpi_ctx, amount_to_treasury)?;

    // Minting to citizen
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.citizen_ata.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    mint_to(cpi_ctx, amount_to_citizen)?;

    // Using checked_add for safety
    governance.spl_minted = governance.spl_minted.checked_add(total_mint_amount as u32)
        .ok_or(ErrorCode::Overflow)?;

        governance.total_sbt_token_supply += 1;


    // Emiting an event for off-chain tracking
    emit!(MintEvent {
        governance: governance.key(),
        treasury_amount: amount_to_treasury,
        citizen_amount: amount_to_citizen,
        total_minted: governance.spl_minted,
    });

    Ok(())
}

#[event]
pub struct MintEvent {
    pub governance: Pubkey,
    pub treasury_amount: u64,
    pub citizen_amount: u64,
    pub total_minted: u32,
}

