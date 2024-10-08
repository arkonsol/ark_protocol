use anchor_lang::prelude::*;
use crate::states::{GovernancePool, GovernanceInvite};
use crate::error::GovernanceError;

#[derive(Accounts)]
pub struct CreateGovernanceInvite<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,

    #[account(
        init,
        payer = creator,
        space = GovernanceInvite::SPACE,
        seeds = [
            b"governance_invite",
            governance_pool.key().as_ref(),
            creator.key().as_ref(),
            &governance_pool.total_participants.to_le_bytes()
        ],
        bump
    )]
    pub invite: Account<'info, GovernanceInvite>,

    #[account(mut, constraint = creator.key() == governance_pool.admin @ GovernanceError::Unauthorized)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_governance_invite(ctx: Context<CreateGovernanceInvite>, expiration_days: u64) -> Result<()> {
    let governance_pool = &ctx.accounts.governance_pool;
    let invite = &mut ctx.accounts.invite;
    let clock = Clock::get()?;

    invite.governance_pool = governance_pool.key();
    invite.creator = ctx.accounts.creator.key();
    invite.created_at = clock.unix_timestamp;
    invite.expires_at = clock.unix_timestamp + (expiration_days as i64 * 24 * 60 * 60);
    invite.is_used = false;
    invite.used_by = None;
    invite.bump = ctx.bumps.invite;

    emit!(GovernanceInviteCreated {
        governance_pool: governance_pool.key(),
        invite: invite.key(),
        creator: invite.creator,
        expires_at: invite.expires_at,
    });

    Ok(())
}

#[event]
pub struct GovernanceInviteCreated {
    pub governance_pool: Pubkey,
    pub invite: Pubkey,
    pub creator: Pubkey,
    pub expires_at: i64,
}

