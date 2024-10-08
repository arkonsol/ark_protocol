use anchor_lang::prelude::*;
use crate::states::{Circle, CircleMemberRecord};
use crate::errors::GovernanceError;


#[derive(Accounts)]
pub struct AddMemberToCircle<'info> {
    #[account(mut)]
    pub circle: Box<Account<'info, Circle>>,

    #[account(
        init,
        payer = payer,
        space = CircleMemberRecord::SPACE,
        seeds = [
            b"circle_member",
            circle.key().as_ref(),
            payer.key().as_ref()
        ],
        bump
    )]
    pub circle_member_record: Box<Account<'info, CircleMemberRecord>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn add_member_to_circle(ctx: Context<AddMemberToCircle>) -> Result<()> {
    let circle = &mut ctx.accounts.circle;
    let circle_member_record = &mut ctx.accounts.circle_member_record;
    let member_pubkey = ctx.accounts.payer.key();

    // Check if the circle has reached its member limit
    require!(
        circle.members.len() < Circle::MAX_MEMBERS,
        GovernanceError::CircleFullyBanner
    );

    // Check if the member is already in the circle
    require!(
        !circle.members.contains(&member_pubkey),
        GovernanceError::MemberAlreadyExists
    );

    // Add member to the circle
    circle.members.push(member_pubkey);

    // Initialize the CircleMemberRecord
    circle_member_record.circle = circle.key();
    circle_member_record.member = member_pubkey;
    circle_member_record.joined_at = Clock::get()?.unix_timestamp;
    circle_member_record.bump = ctx.bumps.circle_member_record;

    // Emit an event
    emit!(MemberAddedToCircle {
        circle: circle.key(),
        member: member_pubkey,
        timestamp: circle_member_record.joined_at,
    });

    Ok(())
}

// Event emitted when a member is added to a circle
#[event]
pub struct MemberAddedToCircle {
    pub circle: Pubkey,
    pub member: Pubkey,
    pub timestamp: i64,
}

// Add this to your state.rs file
