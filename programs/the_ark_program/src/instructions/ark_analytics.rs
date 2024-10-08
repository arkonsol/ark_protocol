use anchor_lang::prelude::*;
use crate::state::analytics::ArkAnalytics;

#[derive(Accounts)]
pub struct InitializeArk<'info> {
    #[account(
        init,
        payer = signer,
        space = ArkAnalytics::LEN,
        seeds = [b"ark_analytics"],
        bump
    )]
    pub ark_analytics: Account<'info, ArkAnalytics>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAnalytics<'info> {
    #[account(mut)]
    pub ark_analytics: Account<'info, ArkAnalytics>,
    #[account(mut)]
    pub signer: Signer<'info>,
}