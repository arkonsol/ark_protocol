use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::{Treasury, Kingdom};

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(
        init, 
        payer = authority, 
        space = Treasury::SPACE,
        seeds = [b"treasury", kingdom.key().as_ref()],
        bump
    )]
    pub treasury: Box<Account<'info, Treasury>>,

    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = treasury
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let kingdom = &mut ctx.accounts.kingdom;

    treasury.balance = 0;
    treasury.taxes_collected = 0;
    treasury.last_collection_date = Clock::get()?.unix_timestamp;
    treasury.royal_expenses = 0;
    treasury.operational_expenses = 0;
    treasury.military_funding = 0;
    treasury.bump = ctx.bumps.treasury;

    kingdom.royal_treasury = treasury.balance;

    Ok(())
}