use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::{Kingdom, Monarch, Subject, EconomicPolicy, EconomicPolicyType};
use anchor_spl::associated_token::AssociatedToken;
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
#[instruction(policy: EconomicPolicyType)]
pub struct SetEconomicPolicy<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(
        init_if_needed,
        payer = authority,
        space = EconomicPolicy::SPACE,
        seeds = [b"economy", kingdom.key().as_ref()],
        bump
    )]
    pub economic_policy: Box<Account<'info, EconomicPolicy>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

pub fn set_economic_policy(
    ctx: Context<SetEconomicPolicy>,
    policy: EconomicPolicyType,
    income_tax_rate: u8,
    property_tax_rate: u8,
    trade_tax_rate: u8,
    luxury_tax_rate: u8
) -> Result<()> {
    require!(
        income_tax_rate <= 100 && property_tax_rate <= 100 && trade_tax_rate <= 100 && luxury_tax_rate <= 100,
        AbsoluteMonarchyError::InvalidTaxRate
    );

    let kingdom = &mut ctx.accounts.kingdom;
    let economic_policy = &mut ctx.accounts.economic_policy;
    let monarch = &mut ctx.accounts.monarch;

    economic_policy.policy_type = policy.clone();
    economic_policy.implemented_at = Clock::get()?.unix_timestamp;
    economic_policy.income_tax_rate = income_tax_rate;
    economic_policy.property_tax_rate = property_tax_rate;
    economic_policy.trade_tax_rate = trade_tax_rate;
    economic_policy.luxury_tax_rate = luxury_tax_rate;
    economic_policy.bump = ctx.bumps.economic_policy;

    monarch.economic_policies_set += 1;
    kingdom.economic_policies_set += 1;

    msg!(
        "New economic policy set: {:?} with tax rates: Income {}%, Property {}%, Trade {}%, Luxury {}%",
        policy, income_tax_rate, property_tax_rate, trade_tax_rate, luxury_tax_rate
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(tax_type: TaxType, taxable_amount: u64)]
pub struct PayTax<'info> {
    #[account(mut)]
    pub kingdom: Account<'info, Kingdom>,

    #[account(
        mut,
        seeds = [b"subject", kingdom.key().as_ref(), &kingdom.total_subjects.to_le_bytes()],
        bump = subject.bump,
    )]
    pub subject: Account<'info, Subject>,

    #[account(
        mut,
        constraint = subject_token_account.owner == subject.key() @ AbsoluteMonarchyError::InvalidTokenAccountOwner
    )]
    pub subject_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_token_account.owner == kingdom.key() @ AbsoluteMonarchyError::InvalidTreasuryAccount
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"economy", kingdom.key().as_ref()],
        bump = economic_policy.bump,
    )]
    pub economic_policy: Account<'info, EconomicPolicy>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum TaxType {
    Income,
    Property,
    Trade,
    Luxury,
}

pub fn pay_tax(ctx: Context<PayTax>, tax_type: TaxType, taxable_amount: u64) -> Result<()> {
    let economic_policy = &ctx.accounts.economic_policy;
    let tax_rate = match tax_type {
        TaxType::Income => economic_policy.income_tax_rate,
        TaxType::Property => economic_policy.property_tax_rate,
        TaxType::Trade => economic_policy.trade_tax_rate,
        TaxType::Luxury => economic_policy.luxury_tax_rate,
    };

    let tax_amount = (taxable_amount as u128 * tax_rate as u128 / 100) as u64;

    // Transfer tax from subject to treasury
    let kingdom = &ctx.accounts.kingdom;
    let kingdom_key = ctx.accounts.kingdom.key();
    let kingdom_subjects = kingdom.total_subjects.to_le_bytes();
    let seeds = &[
        b"subject",
        kingdom_key.as_ref(),
        kingdom_subjects.as_ref(),
        &[ctx.accounts.subject.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let transfer_instruction = Transfer {
        from: ctx.accounts.subject_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.subject.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signer_seeds
        ),
        tax_amount,
    )?;

    // Update subject's wealth
    let subject = &mut ctx.accounts.subject;
    subject.wealth = subject.wealth.saturating_sub(tax_amount);

    msg!("Tax paid: {} for {:?} tax on amount {}", tax_amount, tax_type, taxable_amount);
    Ok(())
}