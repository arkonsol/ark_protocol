use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::{Kingdom, KingdomInvite, Subject};
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct UseKingdomInvite<'info> {
    #[account(mut)]
    pub kingdom: Account<'info, Kingdom>,

    #[account(
        mut,
        constraint = invite.kingdom == kingdom.key() @ AbsoluteMonarchyError::InvalidInvite,
        constraint = !invite.is_used @ AbsoluteMonarchyError::InviteAlreadyUsed,
        constraint = Clock::get()?.unix_timestamp <= invite.expires_at @ AbsoluteMonarchyError::InviteExpired,
    )]
    pub invite: Account<'info, KingdomInvite>,

    #[account(
        init,
        payer = new_subject,
        space = Subject::SPACE,
        seeds = [b"subject", kingdom.key().as_ref(), &kingdom.total_subjects.to_le_bytes()],
        bump
    )]
    pub subject: Account<'info, Subject>,

    #[account(mut)]
    pub new_subject: Signer<'info>,

    #[account(
        init_if_needed,
        payer = new_subject,
        associated_token::mint = kingdom_mint,
        associated_token::authority = new_subject
    )]
    pub member_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub kingdom_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn use_kingdom_invite(ctx: Context<UseKingdomInvite>) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;
    let invite = &mut ctx.accounts.invite;
    let subject = &mut ctx.accounts.subject;
    let new_subject = &ctx.accounts.new_subject;
    let member_token_account = &ctx.accounts.member_token_account;
    let clock = Clock::get()?;

    // Check if the member has the required tokens
    require!(
        member_token_account.amount >= kingdom.membership_tokens_threshold,
        AbsoluteMonarchyError::InsufficientTokens
    );

    // Initialize the subject account
    subject.key = new_subject.key();
    subject.role = "Citizen".to_string();
    subject.jurisdiction = "Kingdom".to_string();
    subject.loyalty = 50; // Start with neutral loyalty
    subject.wealth = 0;
    subject.is_convicted = false;
    subject.appointed_at = clock.unix_timestamp;
    subject.bump = ctx.bumps.subject;

    // Mark the invite as used
    invite.is_used = true;
    invite.used_by = Some(new_subject.key());

    // Update Kingdom state
    kingdom.total_subjects += 1;

    // Check if the kingdom owns the mint authority
    // if kingdom.key() != ctx.accounts.kingdom_mint.mint_authority.unwrap() {
    //     msg!("Warning: The kingdom does not own the mint authority for this token. Anyone who owns the token can join as a member.");
    // }

    emit!(SubjectAddedToKingdom {
        kingdom: kingdom.key(),
        subject: new_subject.key(),
        joined_at: subject.appointed_at,
    });

    Ok(())
}

#[event]
pub struct SubjectAddedToKingdom {
    pub kingdom: Pubkey,
    pub subject: Pubkey,
    pub joined_at: i64,
}