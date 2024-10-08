use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, Token, TokenAccount, Transfer};
use crate::error::AbsoluteMonarchyError;
use crate::{Division, Monarch, Kingdom};
use anchor_spl::associated_token::AssociatedToken;

pub const ONE_MONTH_IN_SECONDS: i64 = 86_400 * 30;

#[derive(Accounts)]
#[instruction(new_division_name: String)]
pub struct ExpandOrganization<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(
        mut,
        constraint = main_treasury.owner == monarch.key() @ AbsoluteMonarchyError::InvalidTreasuryOwner
    )]
    pub main_treasury: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        space = Division::SPACE,
        seeds = [b"division", monarch.key().as_ref(), new_division_name.as_bytes()],
        bump
    )]
    pub division: Box<Account<'info, Division>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = division
    )]
    pub division_treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = authority.key() == monarch.authority @ AbsoluteMonarchyError::NotMonarch
    )]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn expand_organization(ctx: Context<ExpandOrganization>, new_division_name: String, budget: u64) -> Result<()> {
    require!(!new_division_name.is_empty(), AbsoluteMonarchyError::EmptyDivisionName);

    let division = &mut ctx.accounts.division;
    let kingdom = &mut ctx.accounts.kingdom;


    division.name = new_division_name;
    division.manager = *ctx.accounts.authority.key;
    division.established_at = Clock::get()?.unix_timestamp;
    division.last_transfer_at = division.established_at;
    division.treasury = ctx.accounts.division_treasury.key();
    division.bump = ctx.bumps.division;

    // Transfer tokens from main treasury to division treasury
    let transfer_instruction = Transfer {
        from: ctx.accounts.main_treasury.to_account_info(),
        to: ctx.accounts.division_treasury.to_account_info(),
        authority: ctx.accounts.monarch.to_account_info(),
    };

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        ),
        budget,
    )?;

    // Update the monarch's record of divisions (assuming we add this field)
    kingdom.divisions.push(division.key());

    msg!("New division '{}' established with budget {}", division.name, budget);
    Ok(())
}

#[derive(Accounts)]
pub struct TransferRevenue<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(mut)]
    pub division: Box<Account<'info, Division>>,

    #[account(
        mut,
        constraint = division_treasury.key() == division.treasury @ AbsoluteMonarchyError::InvalidTreasury,
        constraint = division_treasury.owner == division.key() @ AbsoluteMonarchyError::InvalidTreasuryOwner
    )]
    pub division_treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    pub main_treasury: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"division", monarch.key().as_ref(), division.name.as_bytes()],
        bump = division.bump
    )]
    /// CHECK: This is safe because we're using it as a signer only
    pub authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn transfer_revenue(ctx: Context<TransferRevenue>) -> Result<()> {
    let division = &mut ctx.accounts.division;
    let monarch = &mut ctx.accounts.monarch;
    let current_time = Clock::get()?.unix_timestamp;
    let monarch_key = monarch.key();
    let division_name = division.name.clone();

    // Check if a month has passed since the last transfer
    if current_time - division.last_transfer_at < ONE_MONTH_IN_SECONDS {
        return Err(AbsoluteMonarchyError::TransferTooSoon.into());
    }

    let division_balance = ctx.accounts.division_treasury.amount;
    let transfer_amount = division_balance / 10; // 10% of the balance

    if transfer_amount == 0 {
        return Err(AbsoluteMonarchyError::InsufficientFunds.into());
    }

    // Transfer 10% from division treasury to main treasury
    let seeds = &[
        b"division",
        monarch_key.as_ref(),
        division_name.as_bytes(),
        &[division.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let transfer_instruction = Transfer {
        from: ctx.accounts.division_treasury.to_account_info(),
        to: ctx.accounts.main_treasury.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signer_seeds
        ),
        transfer_amount,
    )?;

    division.last_transfer_at = current_time;

    msg!("Transferred {} from division '{}' to main treasury", transfer_amount, division.name);

    // Check if division treasury is now empty
    if ctx.accounts.division_treasury.amount == 0 {
        msg!("Division '{}' treasury is empty. Closing division.", division.name);
        
        // Close the division treasury account
        let close_accounts = CloseAccount {
            account: ctx.accounts.division_treasury.to_account_info(),
            destination: ctx.accounts.main_treasury.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            close_accounts,
            signer_seeds
        ))?;

        // Close the division account and transfer lamports to the main treasury
        let destination = ctx.accounts.main_treasury.to_account_info();
        let source = division.to_account_info();
        let dest_starting_lamports = destination.lamports();
        **destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(source.lamports())
            .unwrap();
        **source.lamports.borrow_mut() = 0;

        // Clear the data of the division account
        division.name = String::new();
        division.manager = Pubkey::default();
        division.established_at = 0;
        division.last_transfer_at = 0;
        division.treasury = Pubkey::default();
    }

    Ok(())
}