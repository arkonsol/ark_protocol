use anchor_lang::prelude::*;
use crate::states::{Kingdom, KingdomInvite};
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct CreateKingdomInvite<'info> {
    #[account(mut)]
    pub kingdom: Account<'info, Kingdom>,

    #[account(
        init,
        payer = creator,
        space = KingdomInvite::SPACE,
        seeds = [
            b"kingdom_invite",
            kingdom.key().as_ref(),
            creator.key().as_ref(),
            &kingdom.total_subjects.to_le_bytes()
        ],
        bump
    )]
    pub invite: Account<'info, KingdomInvite>,

    #[account(mut, constraint = creator.key() == kingdom.monarch || kingdom.nobles.contains(&creator.key()) @ AbsoluteMonarchyError::NotMonarch)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_kingdom_invite(ctx: Context<CreateKingdomInvite>, expiration_days: u64) -> Result<()> {
    let kingdom = &ctx.accounts.kingdom;
    let invite = &mut ctx.accounts.invite;
    let clock = Clock::get()?;

    invite.kingdom = kingdom.key();
    invite.creator = ctx.accounts.creator.key();
    invite.created_at = clock.unix_timestamp;
    invite.expires_at = clock.unix_timestamp + (expiration_days as i64 * 24 * 60 * 60);
    invite.is_used = false;
    invite.used_by = None;
    invite.bump = ctx.bumps.invite;

    emit!(KingdomInviteCreated {
        kingdom: kingdom.key(),
        invite: invite.key(),
        creator: invite.creator,
        expires_at: invite.expires_at,
    });

    Ok(())
}

#[event]
pub struct KingdomInviteCreated {
    pub kingdom: Pubkey,
    pub invite: Pubkey,
    pub creator: Pubkey,
    pub expires_at: i64,
}