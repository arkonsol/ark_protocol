use anchor_lang::prelude::*;
use crate::states::{proposal::{Proposal, ProposalStatus}, vote::StakeAccount, governance::Governance};
use anchor_spl::token::{TokenAccount, Token, Transfer, Mint, transfer};
use anchor_spl::associated_token::AssociatedToken;
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct UnstakeFromProposal<'info> {
    #[account(mut)]
    pub governance: Box<Account<'info, Governance>>,
    #[account(mut)]
    pub proposal: Box<Account<'info, Proposal>>,
    #[account(
        mut,
        seeds = [b"stake", proposal.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake: Box<Account<'info, StakeAccount>>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = governance,
    )]
    pub proposal_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn unstake(ctx: Context<UnstakeFromProposal>, amount: u64) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake;
    let proposal = &ctx.accounts.proposal;
    let clock = Clock::get()?;

    require!(proposal.status == ProposalStatus::Active, ErrorCode::ProposalNotActive);
    require!(amount <= stake_account.amount, ErrorCode::InsufficientStake);
    require!(clock.unix_timestamp >= stake_account.lock_end, ErrorCode::StakeLocked);
    let governance_key = ctx.accounts.governance.key();

    let unstake_seeds = &[b"governance", governance_key.as_ref(), &[ctx.accounts.governance.bump]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.proposal_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.governance.to_account_info(),
            },
            &[unstake_seeds]
        ),
        amount,
    )?;

    stake_account.amount -= amount;

    // If all tokens are unstaked, close the stake account
    if stake_account.amount == 0 {
        let stake_account_lamports = ctx.accounts.stake.to_account_info().lamports();
        **ctx.accounts.stake.to_account_info().try_borrow_mut_lamports()? = 0;
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += stake_account_lamports;
    }

    Ok(())
}