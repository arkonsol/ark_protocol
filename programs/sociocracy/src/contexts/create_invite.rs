use anchor_lang::prelude::*;
use crate::states::{Circle, CircleInvite};
use crate::errors::GovernanceError;

#[derive(Accounts)]
pub struct CreateCircleInvite<'info> {
    #[account(mut)]
    pub circle: Box<Account<'info, Circle>>,

    #[account(
        init,
        payer = creator,
        space = CircleInvite::SPACE,
        seeds = [
            b"circle_invite",
            circle.key().as_ref(),
            creator.key().as_ref(),
            &circle.members.len().to_le_bytes()
        ],
        bump
    )]
    pub invite: Box<Account<'info, CircleInvite>>,

    #[account(mut, constraint = circle.members.contains(&creator.key()) @ GovernanceError::NotCircleMember)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_invite(ctx: Context<CreateCircleInvite>, expiration_days: u64) -> Result<()> {
    let circle = &ctx.accounts.circle;
    let invite = &mut ctx.accounts.invite;
    let clock = Clock::get()?;

    // Check if the circle has reached its member limit
    require!(
        circle.members.len() < Circle::MAX_MEMBERS,
        GovernanceError::CircleFullyBanner
    );

    invite.circle = circle.key();
    invite.creator = ctx.accounts.creator.key();
    invite.created_at = clock.unix_timestamp;
    invite.expires_at = clock.unix_timestamp + (expiration_days as i64 * 24 * 60 * 60);
    invite.is_used = false;
    invite.used_by = None;
    invite.bump = ctx.bumps.invite;

    emit!(CircleInviteCreated {
        circle: circle.key(),
        invite: invite.key(),
        creator: invite.creator,
        expires_at: invite.expires_at,
    });

    Ok(())
}

#[event]
pub struct CircleInviteCreated {
    pub circle: Pubkey,
    pub invite: Pubkey,
    pub creator: Pubkey,
    pub expires_at: i64,
}