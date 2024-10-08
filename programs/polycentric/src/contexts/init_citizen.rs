use anchor_lang::prelude::*;
use crate::states::{GovernancePool, Assembly, Citizen};
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::error::GovernanceError; 
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct InitializeCitizen<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,

    #[account(
        init,
        payer = user,
        space = Citizen::SPACE,
        seeds = [b"citizen", governance_pool.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub citizen: Account<'info, Citizen>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn initialize_citizen(ctx: Context<InitializeCitizen>) -> Result<()> {
    let governance_pool = &ctx.accounts.governance_pool;
    let citizen = &mut ctx.accounts.citizen;
    let user = &ctx.accounts.user;

    citizen.governance_pool = governance_pool.key();
    citizen.user = user.key();
    citizen.assemblies = Vec::new();
    citizen.staked_tokens = 0;
    citizen.completed_tasks = 0;
    citizen.voting_power = 0;
    citizen.joined_at = Clock::get()?.unix_timestamp;
    citizen.bump = ctx.bumps.citizen;

    Ok(())
}

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,

    #[account(mut)]
    pub assembly: Account<'info, Assembly>,

    #[account(
        init_if_needed,
        payer = new_member,
        space = Citizen::SPACE,
        seeds = [
            b"citizen",
            governance_pool.key().as_ref(),
            new_member.key().as_ref()
        ],
        bump
    )]
    pub citizen: Account<'info, Citizen>,

    /// The account of the member being added
    #[account(mut)]
    pub new_member: Signer<'info>,

    /// The mint of the governance token
    pub governance_token_mint: Account<'info, Mint>,

    /// The token account of the new member for the governance token
    #[account(
        associated_token::mint = governance_token_mint,
        associated_token::authority = new_member,
    )]
    pub member_governance_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn add_member(ctx: Context<AddMember>) -> Result<()> {
    let governance_pool = &ctx.accounts.governance_pool;
    let assembly = &mut ctx.accounts.assembly;
    let citizen = &mut ctx.accounts.citizen;
    let new_member = &ctx.accounts.new_member;
    let member_token_account = &ctx.accounts.member_governance_token_account;

    // Check if the member has the required tokens
    require!(
        member_token_account.amount >= governance_pool.membership_token_threshold,
        GovernanceError::InsufficientTokens
    );


    require!(
        !assembly.members.contains(&new_member.key()),
        GovernanceError::AlreadyMember
    );

    // Initialize or update the citizen account
    if citizen.user == Pubkey::default() {
        citizen.governance_pool = governance_pool.key();
        citizen.user = new_member.key();
        citizen.joined_at = Clock::get()?.unix_timestamp;
        citizen.bump = ctx.bumps.citizen;
    }

    citizen.assemblies.push(assembly.key());
    citizen.staked_tokens = member_token_account.amount;
    citizen.voting_power = member_token_account.amount; // Simple 1:1 relationship
    
    // Add member to the assembly
    assembly.members.push(new_member.key());

    // Emit an event
    emit!(MemberAdded {
        governance_pool: governance_pool.key(),
        assembly: assembly.key(),
        member: new_member.key(),
        token_count: member_token_account.amount,
    });

    Ok(())
}

#[event]
pub struct MemberAdded {
    pub governance_pool: Pubkey,
    pub assembly: Pubkey,
    pub member: Pubkey,
    pub token_count: u64,
}