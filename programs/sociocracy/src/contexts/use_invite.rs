use anchor_lang::prelude::*;
use crate::states::{Circle, CircleInvite, CircleMemberRecord, Member};
use crate::errors::GovernanceError;

#[derive(Accounts)]
pub struct UseCircleInvite<'info> {
    #[account(mut)]
    pub circle: Box<Account<'info, Circle>>,

    #[account(
        mut,
        constraint = invite.circle == circle.key() @ GovernanceError::InvalidInvite,
        constraint = !invite.is_used @ GovernanceError::InviteAlreadyUsed,
        constraint = Clock::get()?.unix_timestamp <= invite.expires_at @ GovernanceError::InviteExpired,
    )]
    pub invite: Box<Account<'info, CircleInvite>>,

    #[account(
        init_if_needed,
        payer = new_member,
        space = Member::SPACE,
        seeds = [b"member", new_member.key().as_ref()],
        bump
    )]
    pub member: Account<'info, Member>,

    #[account(
        init,
        payer = new_member,
        space = CircleMemberRecord::SPACE,
        seeds = [
            b"circle_member",
            circle.key().as_ref(),
            new_member.key().as_ref()
        ],
        bump
    )]
    pub circle_member_record: Box<Account<'info, CircleMemberRecord>>,

    #[account(mut)]
    pub new_member: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn use_invite(ctx: Context<UseCircleInvite>, name: String) -> Result<()> {
    let circle = &mut ctx.accounts.circle;
    let invite = &mut ctx.accounts.invite;
    let member = &mut ctx.accounts.member;
    let circle_member_record = &mut ctx.accounts.circle_member_record;
    let new_member = &ctx.accounts.new_member;
    let clock = Clock::get()?;

    // Check if the circle has reached its member limit
    require!(
        circle.members.len() < Circle::MAX_MEMBERS,
        GovernanceError::CircleFullyBanner
    );

    // Check if the member is already in the circle
    require!(
        !circle.members.contains(&new_member.key()),
        GovernanceError::MemberAlreadyExists
    );

    // Initialize or update the Member account
    if member.name.is_empty() {
        require!(name.len() <= Member::MAX_NAME_LENGTH, GovernanceError::NameTooLong);
        member.name = name;
        member.bump = ctx.bumps.member;
    }
    if !member.circles.contains(&circle.key()) {
        member.circles.push(circle.key());
    }

    // Add member to the circle
    circle.members.push(new_member.key());

    // Initialize the CircleMemberRecord
    circle_member_record.circle = circle.key();
    circle_member_record.member = new_member.key();
    circle_member_record.joined_at = clock.unix_timestamp;
    circle_member_record.bump = ctx.bumps.circle_member_record;

    // Mark the invite as used
    invite.is_used = true;
    invite.used_by = Some(new_member.key());

    emit!(MemberAddedToCircle {
        circle: circle.key(),
        member: new_member.key(),
        timestamp: circle_member_record.joined_at,
    });

    Ok(())
}

#[event]
pub struct MemberAddedToCircle {
    pub circle: Pubkey,
    pub member: Pubkey,
    pub timestamp: i64,
}