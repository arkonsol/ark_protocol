use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use crate::states::{governance::Governance, proposal::Proposal, vote::{StakeAccount, Vote}};
use crate::states::helpers::*;
use crate::errors::ErrorCode;
#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub governance: Box<Account<'info, Governance>>,
    #[account(mut)]
    pub proposal: Box<Account<'info, Proposal>>,
    #[account(
        init,
        payer = voter,
        space = Vote::SPACE,
        seeds = [b"vote", governance.key().as_ref(), proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,
    #[account(mut)]
    pub stake_account: Account<'info, StakeAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn vote_with_power(ctx: Context<CastVote>, vote: bool, voting_power: u64) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let vote_account = &mut ctx.accounts.vote;
    let governance = &ctx.accounts.governance;
    let stake_account = &ctx.accounts.stake_account;
    let clock = Clock::get()?;

    require!(
        clock.unix_timestamp >= proposal.start_time && clock.unix_timestamp <= proposal.end_time,
        ErrorCode::VotingPeriodInactive
    );

    let max_voting_power = calculate_voting_power(stake_account.amount, stake_account.conviction_multiplier);
    require!(voting_power <= max_voting_power, ErrorCode::ExceedsAvailableVotingPower);

    // Initialize vote account if not already
    if vote_account.vote.is_none() {
        vote_account.governance = governance.key();
        vote_account.proposal = proposal.key();
        vote_account.voter = ctx.accounts.voter.key();
        vote_account.amount = stake_account.amount;
        vote_account.conviction = stake_account.conviction_multiplier as u64;
        vote_account.last_update = clock.unix_timestamp;
        vote_account.bump = ctx.bumps.vote;
    }

    // Remove previous vote if exists
    if let Some(previous_vote) = vote_account.vote {
        if previous_vote {
            proposal.for_votes = proposal.for_votes.saturating_sub(vote_account.power);
        } else {
            proposal.against_votes = proposal.against_votes.saturating_sub(vote_account.power);
        }
    }

    // Record new vote
    if vote {
        proposal.for_votes = proposal.for_votes.saturating_add(voting_power);
    } else {
        proposal.against_votes = proposal.against_votes.saturating_add(voting_power);
    }

    vote_account.vote = Some(vote);
    vote_account.power = voting_power;

    Ok(())
}