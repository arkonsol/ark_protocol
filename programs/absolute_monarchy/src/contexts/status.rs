use anchor_lang::prelude::*;
use crate::states::{Monarch, Subject, Noble, Kingdom};
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct GrantNobility<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(mut)]
    pub subject: Box<Account<'info, Subject>>,

    #[account(
        init,
        payer = authority,
        space = Noble::SPACE,
        seeds = [b"noble", kingdom.key().as_ref()],
        bump
    )]
    pub noble: Box<Account<'info, Noble>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn grant_nobility(ctx: Context<GrantNobility>, title: String) -> Result<()> {
    let subject_account = &mut ctx.accounts.subject;
    let noble = &mut ctx.accounts.noble;
    let kingdom = &mut ctx.accounts.kingdom;


    noble.subject = noble.key();
    noble.title = title;
    noble.granted_at = Clock::get()?.unix_timestamp;

    subject_account.nobility_title = Some(noble.title.clone());
    subject_account.loyalty = subject_account.loyalty.saturating_add(10); // Increase loyalty due to ennoblement
    kingdom.nobles.push(noble.key());
    
    msg!("Nobility granted to {}: {}", noble.subject, noble.title);
    Ok(())
}