use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::states::{Monarch, Kingdom, Subject};
use crate::error::AbsoluteMonarchyError;
use anchor_spl::associated_token::AssociatedToken;


#[derive(Accounts)]
pub struct AppointOfficial<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(
        init,
        payer = authority,
        space = Subject::SPACE,
        seeds = [b"subject", kingdom.key().as_ref(), &kingdom.total_subjects.to_le_bytes()],
        bump
    )]
    pub subject: Box<Account<'info, Subject>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

pub fn appoint_official(ctx: Context<AppointOfficial>, role: String, jurisdiction: String) -> Result<()> {
    let subject_account = &mut ctx.accounts.subject;
    let kingdom = &mut ctx.accounts.kingdom;

    subject_account.key = subject_account.key();
    subject_account.role = role;
    subject_account.jurisdiction = jurisdiction;
    subject_account.loyalty = 100; // Start with full loyalty
    subject_account.wealth = 0;
    subject_account.is_convicted = false;
    subject_account.appointed_at = Clock::get()?.unix_timestamp;
    subject_account.bump = ctx.bumps.subject;

    kingdom.officials_appointed.push(subject_account.key());
    kingdom.total_subjects += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct UnappointOfficial<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(mut, close = authority)]
    pub subject: Account<'info, Subject>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn unappoint_official(ctx: Context<UnappointOfficial>) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;
    let subject_key = ctx.accounts.subject.key();

    kingdom.officials_appointed.retain(|&x| x != subject_key);

    msg!("Official unappointed and removed from the kingdom");

    Ok(())
}

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub kingdom: Account<'info, Kingdom>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Subject::SPACE,
        seeds = [b"subject", kingdom.key().as_ref(), &kingdom.total_subjects.to_le_bytes()],
        bump
    )]
    pub subject: Account<'info, Subject>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = kingdom_mint,
        associated_token::authority = authority
    )]
    pub member_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub kingdom_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn add_member(ctx: Context<AddMember>) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;
    let subject = &mut ctx.accounts.subject;
    let member_token_account = &ctx.accounts.member_token_account;

    // Check if the member has the required tokens
    require!(
        member_token_account.amount >= kingdom.membership_tokens_threshold,
        AbsoluteMonarchyError::InsufficientTokens
    );

    subject.key = subject.key();
    subject.role = "Citizen".to_string();
    subject.jurisdiction = "Kingdom".to_string();
    subject.loyalty = 50; // Start with neutral loyalty
    subject.wealth = 0;
    subject.is_convicted = false;
    subject.appointed_at = Clock::get()?.unix_timestamp;
    subject.bump = ctx.bumps.subject;

    kingdom.total_subjects += 1;

    // Check if the kingdom owns the mint authority
    // if kingdom.key() != ctx.accounts.kingdom_mint.mint_authority.unwrap() {
    //     msg!("Warning: The kingdom does not own the mint authority for this token. Anyone who owns the token can join as a member.");
    // }

    emit!(MemberAdded {
        kingdom: kingdom.key(),
        member: ctx.accounts.authority.key(),
        joined_at: subject.appointed_at,
    });

    Ok(())
}

#[event]
pub struct MemberAdded {
    pub kingdom: Pubkey,
    pub member: Pubkey,
    pub joined_at: i64,
}