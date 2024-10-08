use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::{Subject, PrivilegedAccess, Monarch, Kingdom};
use crate::error::AbsoluteMonarchyError;


#[derive(Accounts)]
pub struct GrantPrivilegedAccess<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(mut)]
    pub beneficiary: Box<Account<'info, Subject>>,

    #[account(
        init,
        payer = authority,
        space = PrivilegedAccess::SPACE,
        seeds = [b"privileges", kingdom.key().as_ref()],
        bump
    )]
    pub privileged_access: Box<Account<'info, PrivilegedAccess>>,

    #[account(
        mut,
        constraint = beneficiary_token_account.owner == beneficiary.key() @ AbsoluteMonarchyError::InvalidTokenAccountOwner
    )]
    pub beneficiary_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

pub fn grant_privileged_access(
    ctx: Context<GrantPrivilegedAccess>, 
    access_type: String, 
    duration: i64, 
    usage_fee_rate: u8,
    access_level: u8,
    initial_fee: u64
) -> Result<()> {
    require!(usage_fee_rate <= 100, AbsoluteMonarchyError::InvalidUsageFeeRate);
    require!(access_level <= 10, AbsoluteMonarchyError::InvalidAccessLevel);

    let kingdom = &ctx.accounts.kingdom;
    let kingdom_key = ctx.accounts.kingdom.key();

    let privileged_access = &mut ctx.accounts.privileged_access;
    privileged_access.access_type = access_type;
    privileged_access.holder = ctx.accounts.beneficiary.key();
    privileged_access.granted_at = Clock::get()?.unix_timestamp;
    privileged_access.expires_at = privileged_access.granted_at + duration;
    privileged_access.usage_fee_rate = usage_fee_rate;
    privileged_access.access_level = access_level;

    let seeds = &[
        b"privileges",
        kingdom_key.as_ref(),
        &kingdom.total_subjects.to_le_bytes(),
        &[ctx.accounts.beneficiary.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    // Transfer initial fee from beneficiary to treasury
    let transfer_instruction = Transfer {
        from: ctx.accounts.beneficiary_token_account.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.beneficiary.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signer_seeds
        ),
        initial_fee,
    )?;

    msg!("Privileged access granted to {} for: {}", privileged_access.holder, privileged_access.access_type);
    msg!("Duration: {} seconds, Usage fee rate: {}%, Access level: {}, Initial fee: {}", 
         duration, usage_fee_rate, access_level, initial_fee);
    Ok(())
}

#[derive(Accounts)]
pub struct UsePrivilegedAccess<'info> {
    pub privileged_access: Account<'info, PrivilegedAccess>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub holder_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn use_privileged_access(ctx: Context<UsePrivilegedAccess>, usage_amount: u64) -> Result<()> {
    let privileged_access = &ctx.accounts.privileged_access;
    let current_time = Clock::get()?.unix_timestamp;

    require!(current_time <= privileged_access.expires_at, AbsoluteMonarchyError::AccessExpired);

    let holder_fee = usage_amount * privileged_access.usage_fee_rate as u64 / 100;
    let treasury_fee = usage_amount / 10; // 10% goes to treasury

    // Transfer fees to holder and treasury
    let transfer_instruction = Transfer {
        from: ctx.accounts.user_account.to_account_info(),
        to: ctx.accounts.holder_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        ),
        holder_fee,
    )?;

    msg!("Holder fee of {} transferred", holder_fee);

    let transfer_instruction = Transfer {
        from: ctx.accounts.user_account.to_account_info(),
        to: ctx.accounts.treasury_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        ),
        treasury_fee,
    )?;

    msg!("Treasury fee of {} transferred", treasury_fee);

    msg!("Privileged access used: {} tokens, {} to holder, {} to treasury", 
         usage_amount, holder_fee, treasury_fee);
    Ok(())
}