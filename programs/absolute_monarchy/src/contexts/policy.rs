use anchor_lang::prelude::*;
use crate::states::{Monarch, Policy, Kingdom};
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct ImplementPolicy<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(
        init,
        payer = authority,
        space = Policy::SPACE,
        seeds = [b"policy", kingdom.key().as_ref()],
        bump
    )]
    pub policy: Box<Account<'info, Policy>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

pub fn implement_policy(
    ctx: Context<ImplementPolicy>, 
    title: String, 
    description: String, 
    target_jurisdiction: String
) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    let monarch = &mut ctx.accounts.monarch;
    let kingdom = &mut ctx.accounts.kingdom;


    require!(title.len() <= Policy::MAXIMUM_TITLE_LENGTH, AbsoluteMonarchyError::PolicyTitleTooLong);
    require!(description.len() <= Policy::MAXIMUM_DESCRIPTION_LENGTH, AbsoluteMonarchyError::PolicyDescriptionTooLong);
    require!(target_jurisdiction.len() <= Policy::MAXIMUM_JURISDICTION_LENGTH, AbsoluteMonarchyError::JurisdictionTooLong);

    policy.id = monarch.policies_implemented + 1;
    policy.title = title;
    policy.description = description;
    policy.target_jurisdiction = target_jurisdiction;
    policy.implemented_at = Clock::get()?.unix_timestamp;
    policy.last_updated_at = policy.implemented_at;
    policy.is_active = true;
    policy.monarch = monarch.key();

    monarch.policies_implemented += 1;
    kingdom.policies_implemented += 1;


    msg!("Policy '{}' implemented in jurisdiction '{}'", policy.title, policy.target_jurisdiction);
    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePolicy<'info> {
    #[account(mut)]
    pub kingdom: Account<'info, Kingdom>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Account<'info, Monarch>,

    #[account(mut, has_one = monarch @ AbsoluteMonarchyError::NotPolicyOwner)]
    pub policy: Account<'info, Policy>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn update_policy(
    ctx: Context<UpdatePolicy>, 
    new_description: Option<String>, 
    new_target_jurisdiction: Option<String>,
    new_is_active: Option<bool>
) -> Result<()> {
    let policy = &mut ctx.accounts.policy;

    if let Some(description) = new_description {
        require!(description.len() <= Policy::MAXIMUM_DESCRIPTION_LENGTH, AbsoluteMonarchyError::PolicyDescriptionTooLong);
        policy.description = description;
    }

    if let Some(jurisdiction) = new_target_jurisdiction {
        require!(jurisdiction.len() <= Policy::MAXIMUM_JURISDICTION_LENGTH, AbsoluteMonarchyError::JurisdictionTooLong);
        policy.target_jurisdiction = jurisdiction;
    }

    if let Some(is_active) = new_is_active {
        policy.is_active = is_active;
    }

    policy.last_updated_at = Clock::get()?.unix_timestamp;

    msg!("Policy '{}' updated", policy.title);
    Ok(())
}