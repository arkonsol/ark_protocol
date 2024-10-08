use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};
use crate::states::{Junta, Citizen};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct AddJuntaMember<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,

    #[account(
        init,
        payer = new_member,
        space = Citizen::LEN,
        seeds = [b"citizen", junta.key().as_ref(), new_member.key().as_ref()],
        bump
    )]
    pub citizen: Account<'info, Citizen>,

    #[account(mut)]
    pub new_member: Signer<'info>,

    #[account(
        associated_token::mint = governance_token_mint,
        associated_token::authority = new_member,
    )]
    pub member_token_account: Account<'info, TokenAccount>,

    pub governance_token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn add_junta_member(ctx: Context<AddJuntaMember>) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let citizen = &mut ctx.accounts.citizen;
    let member_token_account = &ctx.accounts.member_token_account;
    let new_member = &ctx.accounts.new_member;

    // Check if the member is already part of the Junta
    require!(
        citizen.authority == Pubkey::default(),
        ErrorCode::AlreadyMember
    );

    // Check if the member has the required tokens
    require!(
        member_token_account.amount >= junta.support_threshold as u64,
        ErrorCode::InsufficientTokens
    );

    // Initialize the citizen account
    citizen.authority = new_member.key();
    citizen.loyalty_score = 50; // Starting loyalty score
    citizen.resources = member_token_account.amount; // Use token amount as initial resources
    citizen.is_dissident = false;
    citizen.is_imprisoned = false;
    citizen.imprisonment_end = None;
    citizen.joined_at = Clock::get()?.unix_timestamp;
    citizen.bump = ctx.bumps.citizen;

    // Update Junta state
    junta.total_subjects = junta.total_subjects.checked_add(1).ok_or(ErrorCode::ArithmeticError)?;

    // Emit an event
    emit!(MemberAdded {
        junta: junta.key(),
        member: new_member.key(),
        governance_power: member_token_account.amount,
    });

    Ok(())
}

#[event]
pub struct MemberAdded {
    pub junta: Pubkey,
    pub member: Pubkey,
    pub governance_power: u64,
}