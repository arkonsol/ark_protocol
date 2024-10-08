use anchor_lang::prelude::*;
use crate::states::{proposal::{Proposal, ProposalStatus}, vote::StakeAccount};
use anchor_spl::token::{TokenAccount, Token, Transfer, Mint, transfer};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::helpers::*;
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct StakeOnProposal<'info> {
    #[account(mut)]
    pub proposal: Box<Account<'info, Proposal>>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"stake", proposal.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake: Box<Account<'info, StakeAccount>>,
    #[account(
        mut
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = stake,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub proposal_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn stake(ctx: Context<StakeOnProposal>, amount: u64, lock_period: u8) -> Result<()> {
    // let governance = &ctx.accounts.governance;
    let proposal = &mut ctx.accounts.proposal;
    let stake_account = &mut ctx.accounts.stake;
    let clock = Clock::get()?;

    // if let Some(spl_mint) = governance.spl_mint {
    //     require!(ctx.accounts.mint.key() == spl_mint, ErrorCode::InvalidMint);
    // } else {
    //     return Err(ErrorCode::SplMintNotSet.into());
    // }

    // require!(
    //     amount >= governance.min_stake_amount,
    //     ErrorCode::StakeTooLow
    // );

    require!(proposal.status == ProposalStatus::Active, ErrorCode::ProposalNotActive);

    let proposal_key = proposal.key();
    let user_key = ctx.accounts.user.key();

    let stake_seeds = &[
        b"stake",
        proposal_key.as_ref(),
        user_key.as_ref(),
        &[ctx.bumps.stake]
    ];

    // Transfer tokens from user to proposal account
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.proposal_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
            &[stake_seeds]
        ),
        amount,
    )?;

    stake_account.user = ctx.accounts.user.key();
    stake_account.amount += amount;
    stake_account.lock_end = clock.unix_timestamp + calculate_lock_duration(lock_period);
    stake_account.conviction_multiplier = calculate_conviction_multiplier(lock_period);

    Ok(())
}