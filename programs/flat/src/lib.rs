use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, Mint}, associated_token::AssociatedToken};
use the_ark_program::cpi::accounts::{RegisterGovernment, AddTokenToTreasury, CreateTreasury};
use the_ark_program::program::TheArkProgram;
use the_ark_program::state::analytics::ArkAnalytics;
use the_ark_program::cpi::register_government;
use the_ark_program::instructions::register_state::StateInfo;
use the_ark_program::instructions::register_state::GovernmentType;

mod constants;
mod contexts;
mod error;
mod states;

use contexts::*;
use states::*;


declare_id!("9y6z2L9hCjt9p6SuTw1q6vMfz4ZjwvMReMyLquYJNnRp");

#[program]
pub mod flat {
    use super::*;

    pub fn initialize_and_register_government(ctx: Context<Initialize>, name: String) -> Result<()> {
        // Create CPI context
        let cpi_program = ctx.accounts.ark_program.to_account_info();
        let cpi_accounts = RegisterGovernment {
            payer: ctx.accounts.creator.to_account_info(),
            ark_analytics: ctx.accounts.ark_analytics.to_account_info(),
            state_info: ctx.accounts.state_info.to_account_info(),
            government_program: ctx.accounts.government_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        // Make the CPI call
        register_government(cpi_ctx, name, GovernmentType::Democracy)?;
    
        Ok(())
    }

    pub fn create_flatdao_invite(ctx: Context<CreateInvite>, expiration_days: u64) -> Result<()> {
        create_invite(ctx, expiration_days)
    }

    pub fn use_flatdao_invite(ctx: Context<UseInvite>) -> Result<()> {
        use_invite(ctx)
    }

    pub fn init_analytics(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.init_analytics(&ctx.bumps)
    }

    pub fn dao_create(
        ctx: Context<DAOCreate>,
        time: Time,
        threshold: u8,
        min_poll_tokens: u64,
        name: String,
    ) -> Result<()> {
        ctx.accounts
            .dao_create(&ctx.bumps, time, threshold, min_poll_tokens, name)?;
        ctx.accounts.update_analytics()
    }

    pub fn stake_new(ctx: Context<StakeNew>, amount: u64) -> Result<()> {
        ctx.accounts.stake_new(amount)
    }

    pub fn stake_deactivate(ctx: Context<StakeDeactivate>) -> Result<()> {
        ctx.accounts.stake_deactivate()
    }

    pub fn stake_claim(ctx: Context<StakeClaim>) -> Result<()> {
        ctx.accounts.stake_claim()
    }

    pub fn poll_new(ctx: Context<PollNew>, title: String, content: String) -> Result<()> {
        ctx.accounts.poll_new(title, content)?;
        ctx.accounts.update_analytics()
    }

    pub fn vote_new(ctx: Context<VoteNew>, poll: u64, choice: Choice) -> Result<()> {
        ctx.accounts.vote_new(poll, choice)?;
        ctx.accounts.update_analytics()
    }

    pub fn add_new_member(ctx: Context<AddMember>) -> Result<()> {
        add_member(ctx)
    }

    pub fn poll_execute(ctx: Context<PollExecute>, poll: u64) -> Result<()> {
        ctx.accounts.poll_execute(poll)
    }
    pub fn create_treasury(ctx: Context<CreateTreasuryCpi>, name: String) -> Result<()> {
        let cpi_program = ctx.accounts.treasury_program.to_account_info();
        let cpi_accounts = CreateTreasury {
            treasury: ctx.accounts.treasury.to_account_info(),
            owner: ctx.accounts.dao.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        // Use the governance pool's key as the authority
        let authority = ctx.accounts.dao.key();
        
        the_ark_program::cpi::create_government_treasury(cpi_ctx, name, authority)
    }

    pub fn add_token_to_treasury(ctx: Context<AddTokenToTreasuryCpi>) -> Result<()> {
        let cpi_program = ctx.accounts.treasury_program.to_account_info();
        let cpi_accounts = AddTokenToTreasury {
            treasury: ctx.accounts.treasury.to_account_info(),
            token_account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            owner: ctx.accounts.dao.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        the_ark_program::cpi::add_new_token_to_treasury(cpi_ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub ark_analytics: Account<'info, ArkAnalytics>,
    #[account(mut)]
    pub state_info: Account<'info, StateInfo>,
    /// CHECK: This is the program ID of the specific government type
    pub government_program: UncheckedAccount<'info>,
    pub ark_program: Program<'info, TheArkProgram>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateTreasuryCpi<'info> {
    #[account(mut)]
    pub dao: Account<'info, DAO>,
    /// CHECK: This account is checked in the CPI call
    pub treasury: UncheckedAccount<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub treasury_program: Program<'info, TheArkProgram>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct AddTokenToTreasuryCpi<'info> {
    #[account(mut)]
    pub dao: Account<'info, DAO>,
    /// CHECK: This account is checked in the CPI call
    pub treasury: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the CPI call
    pub token_account: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub treasury_program: Program<'info, TheArkProgram>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
