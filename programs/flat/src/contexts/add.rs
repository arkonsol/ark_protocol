use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};
use crate::states::{DAO, UserMembership};
use crate::error::ErrorCode;
use crate::constants::*;

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub dao: Account<'info, DAO>,

    #[account(
        init,
        payer = new_member,
        space = UserMembership::LEN,
        seeds = [b"user_membership", dao.key().as_ref(), new_member.key().as_ref()],
        bump
    )]
    pub user_membership: Account<'info, UserMembership>,

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

pub fn add_member(ctx: Context<AddMember>) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    let user_membership = &mut ctx.accounts.user_membership;
    let member_token_account = &ctx.accounts.member_token_account;
    let new_member = &ctx.accounts.new_member;

    require!(dao.total_members < MAX_TOTAL_MEMBERS, ErrorCode::MaxMembersReached);

    // Initialize the user membership
    user_membership.user = new_member.key();
    user_membership.dao = dao.key();
    user_membership.voting_power = member_token_account.amount;
    user_membership.joined_at = Clock::get()?.unix_timestamp;
    user_membership.bump = ctx.bumps.user_membership;

    // Update DAO state
    dao.total_members = dao.total_members.checked_add(1).ok_or(ErrorCode::ArithmeticError)?;

    // Emit an event
    emit!(MemberAdded {
        dao: dao.key(),
        member: new_member.key(),
        voting_power: member_token_account.amount,
    });

    Ok(())
}

#[event]
pub struct MemberAdded {
    pub dao: Pubkey,
    pub member: Pubkey,
    pub voting_power: u64,
}