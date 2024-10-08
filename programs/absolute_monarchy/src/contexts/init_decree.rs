use anchor_lang::prelude::*;
use crate::states::{Monarch, Decree, DecreeType, Kingdom};
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct DecreeContext<'info> {
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
        space = Decree::SPACE,
        seeds = [b"decree", monarch.key().as_ref()],
        bump
    )]
    pub decree: Box<Account<'info, Decree>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn decree(ctx: Context<DecreeContext>, decree_text: String, decree_type: DecreeType) -> Result<()> {
    let monarch = &mut ctx.accounts.monarch;
    let kingdom = &mut ctx.accounts.kingdom;
    let decree = &mut ctx.accounts.decree;

    decree.id = monarch.decrees_issued + 1;
    decree.text = decree_text;
    decree.decree_type = decree_type;
    decree.issued_at = Clock::get()?.unix_timestamp;
    decree.is_active = true;
    decree.bump = ctx.bumps.decree;

    monarch.decrees_issued += 1;
    kingdom.total_active_decrees += 1;
    kingdom.total_decrees += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct RepealDecree<'info> {
    #[account(mut, has_one = authority @ AbsoluteMonarchyError::NotMonarch)]
    pub monarch: Account<'info, Monarch>,

    #[account(mut)]
    pub decree: Account<'info, Decree>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn repeal_decree(ctx: Context<RepealDecree>, decree_id: u64) -> Result<()> {
    let decree = &mut ctx.accounts.decree;
    require!(decree.id == decree_id, AbsoluteMonarchyError::InvalidDecree);
    decree.is_active = false;
    Ok(())
}