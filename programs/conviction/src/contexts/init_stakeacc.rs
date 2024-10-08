use anchor_lang::prelude::*;
use crate::states::StakeAccount;

#[derive(Accounts)]
pub struct InitializeStakeAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = StakeAccount::SPACE,
        seeds = [b"stake", user.key().as_ref()],
        bump
    )]
    pub stake_account: Box<Account<'info, StakeAccount>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_stake_account(ctx: Context<InitializeStakeAccount>) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    let user = &ctx.accounts.user;

    stake_account.user = user.key();
    stake_account.amount = 0;
    stake_account.lock_end = 1;
    stake_account.conviction_multiplier = 1;
    stake_account.bump = ctx.bumps.stake_account;

    Ok(())
}