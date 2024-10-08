use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::states::{proposal::{Proposal, ProposalStatus}, vote::StakeAccount, governance::Governance};
use crate::states::helpers::*;
use anchor_spl::associated_token::AssociatedToken;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct StakeNftOnProposal<'info> {
    #[account(mut)]
    pub governance: Box<Account<'info, Governance>>,
    #[account(mut)]
    pub proposal: Box<Account<'info, Proposal>>,
    #[account(
        init_if_needed,
        payer = user,
        space = StakeAccount::SPACE,  // Added space for NFT mint
        seeds = [b"nft_stake", proposal.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake: Box<Account<'info, StakeAccount>>,
    #[account(
        mut
    )]
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub user_nft_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = nft_mint,
        associated_token::authority = governance,
    )]
    pub proposal_nft_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn stake_nft(ctx: Context<StakeNftOnProposal>, lock_period: u8) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let stake_account = &mut ctx.accounts.stake;
    let governance = &ctx.accounts.governance;
    let clock = Clock::get()?;

    require!(proposal.status == ProposalStatus::Active, ErrorCode::ProposalNotActive);

    if let Some(nft_mint) = governance.nft_mint {
        require!(ctx.accounts.nft_mint.key() == nft_mint, ErrorCode::InvalidMint);
    } else {
        return Err(ErrorCode::NftMintNotSet.into());
    }

    // Transfer NFT from user to proposal account
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_nft_account.to_account_info(),
                to: ctx.accounts.proposal_nft_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        1, // Transfer 1 NFT
    )?;

    stake_account.user = ctx.accounts.user.key();
    stake_account.nft_amount = 1; // For NFTs, we use 1 as the amount
    stake_account.lock_end = clock.unix_timestamp + calculate_lock_duration(lock_period);
    stake_account.conviction_multiplier = calculate_conviction_multiplier(lock_period);

    Ok(())
}