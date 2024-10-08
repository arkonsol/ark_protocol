use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};
use crate::states::{Junta, JuntaInvite, Citizen};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct UseJuntaInvite<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,

    #[account(
        mut,
        constraint = invite.junta == junta.key() @ ErrorCode::InvalidInvite,
        constraint = !invite.is_used @ ErrorCode::InviteAlreadyUsed,
        constraint = Clock::get()?.unix_timestamp <= invite.expires_at @ ErrorCode::InviteExpired,
    )]
    pub invite: Account<'info, JuntaInvite>,

    #[account(
        init,
        payer = new_member,
        space = Citizen::LEN,
        seeds = [b"citizen", junta.key().as_ref(), new_member.key().as_ref()],
        bump
    )]
    pub citizen: Account<'info, Citizen>,

    #[account(mut)]
    pub new_member: Signer<'info>,

    #[account(
        associated_token::mint = governance_token_mint,
        associated_token::authority = new_member,
    )]
    pub member_token_account: Account<'info, TokenAccount>,

    pub governance_token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn use_junta_invite(ctx: Context<UseJuntaInvite>) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let invite = &mut ctx.accounts.invite;
    let citizen = &mut ctx.accounts.citizen;
    let new_member = &ctx.accounts.new_member;
    let member_token_account = &ctx.accounts.member_token_account;
    let clock = Clock::get()?;

    // Check if the member has the required tokens
    require!(
        member_token_account.amount >= junta.support_threshold as u64,
        ErrorCode::InsufficientTokens
    );

    // Initialize the citizen account
    citizen.authority = new_member.key();
    citizen.loyalty_score = 50; // Starting loyalty score
    citizen.resources = member_token_account.amount; // Use token amount as initial resources
    citizen.is_dissident = false;
    citizen.is_imprisoned = false;
    citizen.imprisonment_end = None;
    citizen.joined_at = clock.unix_timestamp;
    citizen.bump = ctx.bumps.citizen;

    // Mark the invite as used
    invite.is_used = true;
    invite.used_by = Some(new_member.key());

    // Update Junta state
    junta.total_subjects = junta.total_subjects.checked_add(1).ok_or(ErrorCode::ArithmeticError)?;

    emit!(CitizenAddedToJunta {
        junta: junta.key(),
        citizen: new_member.key(),
        governance_power: member_token_account.amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct CitizenAddedToJunta {
    pub junta: Pubkey,
    pub citizen: Pubkey,
    pub governance_power: u64,
    pub timestamp: i64,
}