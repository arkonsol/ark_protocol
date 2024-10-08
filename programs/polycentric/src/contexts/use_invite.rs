use anchor_lang::prelude::*;
use crate::states::{GovernancePool, GovernanceInvite, Citizen};
use crate::error::GovernanceError;
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct UseGovernanceInvite<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,

    #[account(
        mut,
        constraint = invite.governance_pool == governance_pool.key() @ GovernanceError::InvalidInvite,
        constraint = !invite.is_used @ GovernanceError::InviteAlreadyUsed,
        constraint = Clock::get()?.unix_timestamp <= invite.expires_at @ GovernanceError::InviteExpired,
    )]
    pub invite: Account<'info, GovernanceInvite>,

    #[account(
        init,
        payer = new_member,
        space = Citizen::SPACE,
        seeds = [b"citizen", governance_pool.key().as_ref(), new_member.key().as_ref()],
        bump
    )]
    pub citizen: Account<'info, Citizen>,

    #[account(mut)]
    pub new_member: Signer<'info>,

    pub governance_token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = new_member,
        associated_token::mint = governance_token_mint,
        associated_token::authority = new_member,
    )]
    pub member_governance_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn use_governance_invite(ctx: Context<UseGovernanceInvite>) -> Result<()> {
    let governance_pool = &mut ctx.accounts.governance_pool;
    let invite = &mut ctx.accounts.invite;
    let citizen = &mut ctx.accounts.citizen;
    let new_member = &ctx.accounts.new_member;
    let clock = Clock::get()?;

    // Initialize the citizen (similar to initialize_citizen function)
    citizen.governance_pool = governance_pool.key();
    citizen.user = new_member.key();
    citizen.assemblies = Vec::new();
    citizen.staked_tokens = 0;
    citizen.completed_tasks = 0;
    citizen.voting_power = 0;
    citizen.joined_at = clock.unix_timestamp;
    citizen.bump = ctx.bumps.citizen;

    // Mark the invite as used
    invite.is_used = true;
    invite.used_by = Some(new_member.key());

    // Update governance pool stats
    governance_pool.total_participants += 1;

    emit!(CitizenAddedToGovernancePool {
        governance_pool: governance_pool.key(),
        citizen: new_member.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct CitizenAddedToGovernancePool {
    pub governance_pool: Pubkey,
    pub citizen: Pubkey,
    pub timestamp: i64,
}