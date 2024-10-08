use anchor_lang::prelude::*;
use crate::states::{Junta, JuntaInvite};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct CreateJuntaInvite<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,

    #[account(
        init,
        payer = creator,
        space = JuntaInvite::SPACE,
        seeds = [
            b"junta_invite",
            junta.key().as_ref(),
            creator.key().as_ref(),
            &junta.total_subjects.to_le_bytes()
        ],
        bump
    )]
    pub invite: Account<'info, JuntaInvite>,

    #[account(mut, constraint = creator.key() == junta.leader || junta.officers.contains(&creator.key()) @ ErrorCode::Unauthorized)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_junta_invite(ctx: Context<CreateJuntaInvite>, expiration_days: u64) -> Result<()> {
    let junta = &ctx.accounts.junta;
    let invite = &mut ctx.accounts.invite;
    let clock = Clock::get()?;

    invite.junta = junta.key();
    invite.creator = ctx.accounts.creator.key();
    invite.created_at = clock.unix_timestamp;
    invite.expires_at = clock.unix_timestamp + (expiration_days as i64 * 24 * 60 * 60);
    invite.is_used = false;
    invite.used_by = None;
    invite.bump = ctx.bumps.invite;

    emit!(JuntaInviteCreated {
        junta: junta.key(),
        invite: invite.key(),
        creator: invite.creator,
        expires_at: invite.expires_at,
    });

    Ok(())
}

#[event]
pub struct JuntaInviteCreated {
    pub junta: Pubkey,
    pub invite: Pubkey,
    pub creator: Pubkey,
    pub expires_at: i64,
}