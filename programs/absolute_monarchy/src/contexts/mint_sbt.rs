use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::kingdom::{Kingdom, InitializeSbtArgs};
use crate::states::subject::Subject;
use the_ark_program::{initialize_token_metadata_extension, mint_to_token_account, set_account_or_mint_authority, update_account_lamports_to_minimum_balance, initialize_token_group_extension, initialize_non_transferrable_extension};

#[derive(Accounts)]
pub struct MintSbt<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,
    
    #[account(
        init,
        payer = payer,
        mint::authority = payer,
        mint::token_program = token_program,
        mint::decimals = 0,
        extensions::metadata_pointer::metadata_address = mint,
        extensions::group_pointer::group_address = mint,
        extensions::transfer_hook::authority = payer,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

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
    pub subject_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

pub fn mint_sbt(ctx: Context<MintSbt>, args: InitializeSbtArgs) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;
    let mint = &ctx.accounts.mint;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let subject_token_account = &ctx.accounts.subject_token_account;

    initialize_token_group_extension(
        u32::MAX,
        mint,
        mint,
        payer,
        Some(subject_token_account.key()),
        token_program,
        None,
    )?;

    initialize_token_metadata_extension(
        mint,
        mint,
        payer,
        payer,
        &ctx.accounts.token_program.to_account_info(),
        args.name.clone(),
        args.symbol.clone(),
        args.uri.clone(),
    )?;

    update_account_lamports_to_minimum_balance(
        mint.to_account_info(),
        payer.to_account_info(),
        system_program.to_account_info(),
    )?;

    mint_to_token_account(1, mint, payer, subject_token_account, token_program)?;

    set_account_or_mint_authority(
        mint,
        &payer.to_account_info(),
        None,
        AuthorityType::MintTokens,
        token_program,
    )?;

    initialize_non_transferrable_extension(mint, token_program)?;

    kingdom.sbt_mint = Some(ctx.accounts.mint.key());
    kingdom.sbt_minted += 1;

    // Update subject information
    let subject = &mut ctx.accounts.subject;
    subject.key = subject.key();
    subject.role = "Citizen".to_string();
    subject.jurisdiction = "Kingdom".to_string();
    subject.loyalty = 0; // Convicted subjects have no loyalty
    subject.is_convicted = true;
    subject.appointed_at = Clock::get()?.unix_timestamp;

    kingdom.total_subjects += 1;

    emit!(SbtMinted {
        kingdom: kingdom.key(),
        mint: ctx.accounts.mint.key(),
        subject: subject.key(),
        name: args.name,
        symbol: args.symbol,
        uri: args.uri,
    });

    Ok(())
}

#[event]
pub struct SbtMinted {
    pub kingdom: Pubkey,
    pub mint: Pubkey,
    pub subject: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}