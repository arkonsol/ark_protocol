use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::states::{DAOInvite, DAO, User};

#[derive(Accounts)]
pub struct CreateInvite<'info> {
    #[account(mut)]
    pub dao: Account<'info, DAO>,
    #[account(
        init,
        payer = creator,
        space = DAOInvite::LEN,
        seeds = [b"invite", dao.key().as_ref(), creator.key().as_ref(), &dao.total_members.to_le_bytes()],
        bump
    )]
    pub invite: Account<'info, DAOInvite>,
    #[account(mut, constraint = creator.key() == dao.creator @ ErrorCode::Unauthorized)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_invite(ctx: Context<CreateInvite>, expiration_days: u64) -> Result<()> {
    let dao = &ctx.accounts.dao;
    let invite = &mut ctx.accounts.invite;
    let clock: Clock = Clock::get().unwrap();

    invite.dao = dao.key();
    invite.creator = ctx.accounts.creator.key();
    invite.created_at = clock.unix_timestamp;
    invite.expires_at = clock.unix_timestamp + (expiration_days as i64 * 24 * 60 * 60);
    invite.is_used = false;
    invite.used_by = None;

    Ok(())
}

#[derive(Accounts)]
pub struct UseInvite<'info> {
    #[account(mut)]
    pub dao: Account<'info, DAO>,
    #[account(mut, constraint = invite.dao == dao.key() @ ErrorCode::InvalidInvite)]
    pub invite: Account<'info, DAOInvite>,
    #[account(mut)]
    pub new_member: Signer<'info>,
}

pub fn use_invite(ctx: Context<UseInvite>) -> Result<()> {
    let invite = &mut ctx.accounts.invite;
    let dao = &mut ctx.accounts.dao;
    let clock: Clock = Clock::get().unwrap();

    require!(!invite.is_used, ErrorCode::InviteAlreadyUsed);
    require!(clock.unix_timestamp <= invite.expires_at, ErrorCode::InviteExpired);

    invite.is_used = true;
    invite.used_by = Some(ctx.accounts.new_member.key());

    // Add the new member to the DAO
    dao.users.push(User {
        user: ctx.accounts.new_member.key(),
        deposits: vec![],
        points: 0,
        voting_power: 0,
        created_at: clock.unix_timestamp,
    });
    dao.total_members += 1;

    Ok(())
}