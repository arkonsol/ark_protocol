use anchor_lang::prelude::*;
use crate::states::{GovernancePool, Task, Assembly, VotingPowerAction, Citizen};
use anchor_spl::token::{Transfer, Token, TokenAccount, transfer};
use crate::error::GovernanceError; 


#[derive(Accounts)]
pub struct ObtainVotingPower<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,
    
    #[account(mut)]
    pub citizen: Account<'info, Citizen>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Optional accounts, only needed for specific actions
    pub assembly: Option<Account<'info, Assembly>>,
    pub task: Option<Account<'info, Task>>,
    #[account(mut)]
    pub user_token_account: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_token_account: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to_citizen: Option<Account<'info, Citizen>>,
    
    pub token_program: Option<Program<'info, Token>>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn obtain_voting_power(ctx: Context<ObtainVotingPower>, action: VotingPowerAction) -> Result<()> {
    let citizen = &mut ctx.accounts.citizen;
    let user = &ctx.accounts.user;

    match action {
        VotingPowerAction::JoinAssembly => {
            let assembly = ctx.accounts.assembly.as_mut().ok_or(GovernanceError::MissingAccount)?;
            require!(!assembly.members.contains(&user.key()), GovernanceError::AlreadyMember);
            assembly.members.push(user.key());
            citizen.assemblies.push(assembly.key());
        },
        VotingPowerAction::CompleteTask { task_id } => {
            let task = ctx.accounts.task.as_mut().ok_or(GovernanceError::MissingAccount)?;
            require!(task.id == task_id, GovernanceError::InvalidTask);
            require!(task.completed_by.is_none(), GovernanceError::TaskAlreadyCompleted);
            // Perform offchain verification, but for now we will keep it simple
            task.completed_by = Some(user.key());
            citizen.completed_tasks += 1;
            citizen.voting_power += task.reward;
        },
        VotingPowerAction::StakeTokens { amount } => {
            let user_token_account = ctx.accounts.user_token_account.as_ref().ok_or(GovernanceError::MissingAccount)?;
            let pool_token_account = ctx.accounts.pool_token_account.as_ref().ok_or(GovernanceError::MissingAccount)?;
            let token_program = ctx.accounts.token_program.as_ref().ok_or(GovernanceError::MissingAccount)?;

            let cpi_accounts = Transfer {
                from: user_token_account.to_account_info(),
                to: pool_token_account.to_account_info(),
                authority: user.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);
            transfer(cpi_ctx, amount)?;

            citizen.staked_tokens += amount;
        },
        VotingPowerAction::DelegatePower { amount } => {
            let to_citizen = ctx.accounts.to_citizen.as_mut().ok_or(GovernanceError::MissingAccount)?;
            require!(citizen.voting_power >= amount, GovernanceError::InsufficientVotingPower);

            citizen.voting_power -= amount;
            citizen.delegated_power += amount;
            to_citizen.voting_power += amount;
        },
    }

    citizen.calculate_voting_power()?;

    Ok(())
}
