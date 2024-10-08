use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use anchor_spl::associated_token::AssociatedToken;

use crate::states::{junta::Junta, citizen::Citizen, supporters::*};

// const SUPPORT_THRESHOLD: u64 = 1000; // Minimum token amount to be considered a supporter
const SUPPORTERS_PER_LEVEL: usize = 10; // Number of supporters needed to increase control level

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct GainSupport<'info> {
    #[account(mut)]
    pub junta: Box<Account<'info, Junta>>,

    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + Supporters::SIZE,
        seeds = [b"supporter", junta.key().as_ref(), &junta.total_subjects.to_le_bytes()],
        bump
    )]
    pub supporters: Box<Account<'info, Supporters>>,

    #[account(
        mut, 
        seeds = [b"citizen", junta.key().as_ref(), &junta.total_subjects.to_le_bytes()],
        bump = citizen.bump
    )]
    pub citizen: Box<Account<'info, Citizen>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>, 

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = citizen,
    )]
    pub citizen_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = junta_token_account.owner == junta.key(),
    )]
    pub junta_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn gain_support(ctx: Context<GainSupport>, amount: u64) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let supporters = &mut ctx.accounts.supporters;

    // Initialize the Supporters account if it's new
    if supporters.count == 0 {
        supporters.supporters = [None; MAX_SUPPORTERS];
        supporters.support_amounts = [(Pubkey::default(), 0); MAX_SUPPORT_AMOUNTS];
    }

    let junta_key = junta.key();
    let junta_subjects = junta.total_subjects.to_le_bytes();
    let seeds = &[
        b"citizen",
        junta_key.as_ref(),
        junta_subjects.as_ref(),
        &[junta.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    // Transfer tokens from citizen to junta
    let cpi_accounts = Transfer {
        from: ctx.accounts.citizen_token_account.to_account_info(),
        to: ctx.accounts.junta_token_account.to_account_info(),
        authority: ctx.accounts.citizen.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    token::transfer(cpi_ctx, amount)?;

    // Check if the citizen has transferred enough to be considered a supporter
    let citizen_key = ctx.accounts.citizen.key();
    let citizen_total_support = supporters.get_support_amount(&citizen_key) + amount;
    
    if citizen_total_support >= junta.support_threshold.into() && !supporters.is_supporter(&citizen_key) {
        supporters.add_supporter(citizen_key)?;
        
        // Calculate how many levels to increase
        let new_levels = (supporters.count as usize / SUPPORTERS_PER_LEVEL) - ((supporters.count - 1) as usize / SUPPORTERS_PER_LEVEL);
        
        // Increase the control level of the junta
        junta.control_level = junta.control_level.saturating_add(new_levels as u8);

        // Emit event for new supporter
        emit!(NewSupporterEvent {
            junta: junta.key(),
            supporter: citizen_key,
            amount: citizen_total_support,
        });
    }

    msg!("New Supporter has been added");

    // Update the citizen's total support amount
    supporters.update_support_amount(citizen_key, citizen_total_support)?;

    // Emit event for support contribution
    emit!(SupportContributionEvent {
        junta: junta.key(),
        supporter: citizen_key,
        amount,
        total_support: citizen_total_support,
    });

    Ok(())
}

#[event]
pub struct NewSupporterEvent {
    pub junta: Pubkey,
    pub supporter: Pubkey,
    pub amount: u64,
}

#[event]
pub struct SupportContributionEvent {
    pub junta: Pubkey,
    pub supporter: Pubkey,
    pub amount: u64,
    pub total_support: u64,
}
