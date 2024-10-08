use anchor_lang::prelude::*;
use crate::states::{Monarch, Kingdom};
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct Abdicate<'info> {
    #[account(
        mut,
        has_one = monarch @ AbsoluteMonarchyError::InvalidMonarch,
    )]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch,
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init, 
        payer = authority, 
        space = Monarch::SPACE,
        seeds = [b"monarch", kingdom.key().as_ref()],
        bump
    )]
    pub new_monarch: Box<Account<'info, Monarch>>,

    pub system_program: Program<'info, System>,
}

pub fn abdicate(ctx: Context<Abdicate>, heir_name: String) -> Result<()> {
    require!(!heir_name.is_empty(), AbsoluteMonarchyError::EmptyHeirName);

    let old_monarch = &mut ctx.accounts.monarch;
    let new_monarch = &mut ctx.accounts.new_monarch;
    let kingdom = &mut ctx.accounts.kingdom;

    // Update the kingdom with the new monarch
    kingdom.monarch = new_monarch.key();
    
    // Initialize the new monarch
    new_monarch.authority = new_monarch.key();
    new_monarch.name = heir_name;
    new_monarch.coronation_date = Clock::get()?.unix_timestamp;
    new_monarch.decrees_issued = 0;
    new_monarch.wars_declared = 0;
    new_monarch.royal_judgments = 0;
    new_monarch.economic_policies_set = 0;
    new_monarch.bump = ctx.bumps.new_monarch;

    old_monarch.abdication_date = Some(Clock::get()?.unix_timestamp);

    msg!("Monarch {} has abdicated. Long live Monarch {}!", old_monarch.name, new_monarch.name);

    Ok(())
}