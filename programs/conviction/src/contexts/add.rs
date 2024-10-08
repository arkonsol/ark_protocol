use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};
use crate::states::{Governance, StakeAccount};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,

    #[account(
        init_if_needed,
        payer = new_member,
        space = StakeAccount::SPACE,
        seeds = [b"stake_account", governance.key().as_ref(), new_member.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub new_member: Signer<'info>,

    #[account(
        associated_token::mint = governance_token_mint,
        associated_token::authority = new_member,
    )]
    pub member_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub governance_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub governance_nft_mint: Account<'info, Mint>,

    #[account(
        associated_token::mint = governance_nft_mint,
        associated_token::authority = new_member,
    )]
    pub member_nft_account: Option<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn add_member(ctx: Context<AddMember>) -> Result<()> {
    let governance = &mut ctx.accounts.governance;
    let stake_account = &mut ctx.accounts.stake_account;
    let new_member = &ctx.accounts.new_member;
    let member_token_account = &ctx.accounts.member_token_account;

    // Check if the member has the required tokens
    require!(
        member_token_account.amount >= governance.min_stake_amount,
        ErrorCode::InsufficientTokens
    );

    // Initialize or update the stake account
    if stake_account.user == Pubkey::default() {
        stake_account.user = new_member.key();
        stake_account.amount = 0;
        stake_account.nft_amount = 0;
        stake_account.lock_end = 0;
        stake_account.conviction_multiplier = 1;
        stake_account.bump = ctx.bumps.stake_account;
    }

    // Update stake amount
    stake_account.amount = member_token_account.amount;

    // If NFT is part of governance, check and update NFT stake
    if let Some(nft_mint) = governance.nft_mint {
        require!(ctx.accounts.governance_nft_mint.key() == nft_mint, ErrorCode::InvalidMint);
    } else {
        return Err(ErrorCode::NftMintNotSet.into());
    }
    
    if let Some(nft_account) = &ctx.accounts.member_nft_account {
        stake_account.nft_amount = nft_account.amount;
    }

    // Update governance stats
    governance.total_members = governance.total_members.checked_add(1).unwrap();

    // Emit an event
    emit!(MemberAdded {
        governance: governance.key(),
        member: new_member.key(),
        staked_amount: stake_account.amount,
        staked_nft_amount: stake_account.nft_amount,
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