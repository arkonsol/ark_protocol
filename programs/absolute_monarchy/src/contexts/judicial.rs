use anchor_lang::prelude::*;
use crate::states::{Monarch, Subject, Kingdom};
use crate::error::AbsoluteMonarchyError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Verdict {
    Guilty,
    Innocent,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Punishment {
    Fine(u64),
    Imprisonment(u64), // Duration in seconds
    Exile,
    Execution,
}

#[derive(Accounts)]
pub struct RoyalJudgment<'info> {
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

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn royal_judgment(ctx: Context<RoyalJudgment>, verdict: Verdict, punishment: Option<Punishment>) -> Result<()> {
    let subject_account = &mut ctx.accounts.subject;

    match verdict {
        Verdict::Guilty => {
            subject_account.is_convicted = true;
            subject_account.loyalty = subject_account.loyalty.saturating_sub(20);

            if let Some(p) = punishment {
                match p {
                    Punishment::Fine(amount) => {
                        require!(subject_account.wealth >= amount, AbsoluteMonarchyError::InsufficientFunds);
                        subject_account.wealth -= amount;
                        msg!("Subject fined {} tokens", amount);
                    },
                    Punishment::Imprisonment(duration) => {
                        // Implement imprisonment logic
                        msg!("Subject imprisoned for {} seconds", duration);
                    },
                    Punishment::Exile => {
                        subject_account.role = "Exiled".to_string();
                        subject_account.jurisdiction = "None".to_string();
                        msg!("Subject exiled");
                    },
                    Punishment::Execution => {
                        // Implement execution logic - close account
                        msg!("Subject executed");
                    },
                }
            }
        },
        Verdict::Innocent => {
            subject_account.is_convicted = false;
            subject_account.loyalty = subject_account.loyalty.saturating_add(10);
            msg!("Subject found innocent");
        },
    }

    ctx.accounts.monarch.royal_judgments += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct RoyalPardon<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(mut)]
    pub subject: Account<'info, Subject>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn royal_pardon(ctx: Context<RoyalPardon>) -> Result<()> {
    let subject_account = &mut ctx.accounts.subject;
    require!(subject_account.is_convicted, AbsoluteMonarchyError::SubjectNotConvicted);

    // Pardon the subject
    subject_account.is_convicted = false;

    // Increase loyalty as a result of the pardon
    subject_account.loyalty = subject_account.loyalty.saturating_add(20);

    // Restore some privileges or status
    if subject_account.role == "Exiled" {
        subject_account.role = "Citizen".to_string();
        subject_account.jurisdiction = "Kingdom".to_string(); // Or any default jurisdiction
    }

    // Record the pardon in the monarch's account
    ctx.accounts.monarch.pardons_granted += 1;

    msg!("Subject {} pardoned by royal decree", subject_account.key);
    msg!("Subject's new loyalty: {}", subject_account.loyalty);
    msg!("Subject's role restored to: {}", subject_account.role);
    
    Ok(())
}