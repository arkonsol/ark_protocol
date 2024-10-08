use anchor_lang::prelude::*;

use anchor_spl::token::{self, TokenAccount, Token, Approve, Mint};
use anchor_spl::associated_token::AssociatedToken;

use crate::states::{Citizen, Junta};

#[derive(Accounts)]
pub struct ApproveDelegate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(mut)]
    pub mint: Account<'info, Mint>, 
    #[account(
        mut, 
        seeds = [b"citizen", junta.key().as_ref(), &junta.total_subjects.to_le_bytes()],
        bump = citizen.bump
    )]
    pub citizen: Box<Account<'info, Citizen>>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = citizen,
    )]
    pub citizen_ata: Account<'info, TokenAccount>,  
    #[account(signer)]
    /// CHECK: We are passing in this account ourselves
    pub leader: UncheckedAccount<'info>,  
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn approve_delegate(ctx: Context<ApproveDelegate>, amount: u64) -> Result<()> {
    let cpi_accounts = Approve {
        to: ctx.accounts.citizen_ata.to_account_info(),
        delegate: ctx.accounts.leader.to_account_info(),
        authority: ctx.accounts.citizen.to_account_info(),
    };

    let junta = &ctx.accounts.junta;
    let junta_key = ctx.accounts.junta.key();
    let junta_subjects = junta.total_subjects.to_le_bytes();
    let seeds = &[
        b"citizen",
        junta_key.as_ref(),
        junta_subjects.as_ref(),
        &[ctx.accounts.junta.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    token::approve(cpi_ctx, amount)?;

    Ok(())
}
