use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::{StakeAccount, ConvictionInvite, Governance};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct CreateInvite<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,
    #[account(
        init,
        payer = creator,
        space = ConvictionInvite::SPACE,
        seeds = [b"invite", governance.key().as_ref(), creator.key().as_ref(), &governance.total_members.to_le_bytes()],
        bump
    )]
    pub invite: Account<'info, ConvictionInvite>,
    #[account(mut, constraint = creator.key() == governance.creator @ ErrorCode::Unauthorized)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_invite(ctx: Context<CreateInvite>, expiration_days: u64) -> Result<()> {
    let governance = &ctx.accounts.governance;
    let invite = &mut ctx.accounts.invite;
    let clock: Clock = Clock::get()?;

    invite.governance = governance.key();
    invite.creator = ctx.accounts.creator.key();
    invite.created_at = clock.unix_timestamp;
    invite.expires_at = clock.unix_timestamp + (expiration_days as i64 * 24 * 60 * 60);
    invite.is_used = false;
    invite.used_by = None;

    // Update the governance stats
    let governance = &mut ctx.accounts.governance;
    governance.total_members += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct UseInvite<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,

    #[account(
        mut,
        seeds = [b"invite", governance.key().as_ref(), invite.creator.as_ref(), &invite.created_at.to_le_bytes()],
        bump,
        constraint = !invite.is_used @ ErrorCode::InviteAlreadyUsed,
        constraint = Clock::get()?.unix_timestamp <= invite.expires_at @ ErrorCode::InviteExpired,
    )]
    pub invite: Account<'info, ConvictionInvite>,

    #[account(
        init,
        payer = new_member,
        space = StakeAccount::SPACE,
        seeds = [b"stake_account", governance.key().as_ref(), new_member.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub new_member: Signer<'info>,

    #[account(
        init_if_needed,
        payer = new_member,
        associated_token::mint = governance_token_mint,
        associated_token::authority = new_member,
    )]
    pub member_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub governance_token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = new_member,
        associated_token::mint = governance_nft_mint,
        associated_token::authority = new_member,
    )]
    pub member_nft_account: Option<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub governance_nft_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn use_invite(ctx: Context<UseInvite>) -> Result<()> {
    let governance = &mut ctx.accounts.governance;
    let invite = &mut ctx.accounts.invite;
    let stake_account = &mut ctx.accounts.stake_account;
    let new_member = &ctx.accounts.new_member;

    // Mark the invite as used
    invite.is_used = true;
    invite.used_by = Some(new_member.key());

    // Initialize the stake account
    stake_account.user = new_member.key();
    stake_account.amount = 0;
    stake_account.nft_amount = 0;
    stake_account.lock_end = 0;
    stake_account.conviction_multiplier = 1;
    stake_account.bump = ctx.bumps.stake_account;

    // Update governance stats
    governance.total_members = governance.total_members.checked_add(1).unwrap();

    // Emit an event
    emit!(MemberAdded {
        governance: governance.key(),
        member: new_member.key(),
        staked_amount: 0,
        staked_nft_amount: 0,
    });

    Ok(())
}

#[event]
pub struct MemberAdded {
    pub governance: Pubkey,
    pub member: Pubkey,
    pub staked_amount: u64,
    pub staked_nft_amount: u64,
}