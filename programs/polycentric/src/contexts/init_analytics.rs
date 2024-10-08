use anchor_lang::prelude::*;
use crate::states::{Analytics, GovernancePool};

#[derive(Accounts)]
pub struct InitializeAnalytics<'info> {
    #[account(
        init,
        payer = admin,
        space = Analytics::LEN,
        seeds = [b"analytics"],
        bump
    )]
    pub analytics: Account<'info, Analytics>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

#[derive(Accounts)]
pub struct UpdateAnalytics<'info> {
    #[account(mut)]
    pub analytics: Account<'info, Analytics>,

    #[account(
        mut,
        seeds = [b"governance_pool", admin.key().as_ref()],
        bump =  governance_pool.bump,
    )]
    pub governance_pool: Account<'info, GovernancePool>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
}

pub fn initialize_analytics(ctx: Context<InitializeAnalytics>) -> Result<()> {
    let analytics = &mut ctx.accounts.analytics;
    analytics.total_governance_pools = 0;
    analytics.total_assemblies = 0;
    analytics.total_policy_areas = 0;
    analytics.total_proposals = 0;
    analytics.total_votes = 0;
    analytics.last_updated = 0;
    analytics.counted_pools = Vec::new();
    analytics.bump = ctx.bumps.analytics;
    Ok(())
}

pub fn update_analytics(ctx: Context<UpdateAnalytics>) -> Result<()> {
    let analytics = &mut ctx.accounts.analytics;
    let governance_pool = &ctx.accounts.governance_pool;
    let clock = &ctx.accounts.clock;

    // Check if this governance pool has been counted before
    if !analytics.counted_pools.contains(&governance_pool.key()) {
        analytics.total_governance_pools += 1;
        analytics.counted_pools.push(governance_pool.key());
    }

    // Update total counts
    analytics.total_assemblies = analytics.total_assemblies.saturating_add(governance_pool.assemblies.len() as u64);
    analytics.total_policy_areas = analytics.total_policy_areas.saturating_add(governance_pool.policy_areas.len() as u64);
    analytics.total_proposals = analytics.total_proposals.saturating_add(governance_pool.total_proposals);
    analytics.total_votes = analytics.total_votes.saturating_add(governance_pool.total_votes);

    // Update the timestamp
    analytics.update_timestamp(clock.unix_timestamp);

    // Emit an event for off-chain tracking
    emit!(AnalyticsUpdated {
        governance_pool: governance_pool.key(),
        total_governance_pools: analytics.total_governance_pools,
        total_assemblies: analytics.total_assemblies,
        total_policy_areas: analytics.total_policy_areas,
        total_proposals: analytics.total_proposals,
        total_votes: analytics.total_votes,
        timestamp: analytics.last_updated,
    });

    Ok(())
}

#[event]
pub struct AnalyticsUpdated {
    pub governance_pool: Pubkey,
    pub total_governance_pools: u64,
    pub total_assemblies: u64,
    pub total_policy_areas: u64,
    pub total_proposals: u64,
    pub total_votes: u64,
    pub timestamp: i64,
}