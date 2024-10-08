use anchor_lang::prelude::*;
use crate::states::{Citizen, Junta};

#[derive(Accounts)]
pub struct InitializeCitizen<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(
        init, 
        payer = authority, 
        space = Citizen::LEN, 
        seeds = [b"citizen", junta.key().as_ref(), &junta.total_subjects.to_le_bytes()],
        bump
    )]
    pub citizen: Box<Account<'info, Citizen>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

pub fn initialize_citizen(ctx: Context<InitializeCitizen>, authority: Pubkey) -> Result<()> {
    let citizen = &mut ctx.accounts.citizen;
    let junta = &mut ctx.accounts.junta;


    citizen.authority = authority;
    citizen.loyalty_score = 50; 
    citizen.resources = 0; 
    citizen.is_dissident = false;
    citizen.is_imprisoned = false;
    citizen.imprisonment_end = None;
    citizen.bump = ctx.bumps.citizen;
    citizen.joined_at = Clock::get()?.unix_timestamp;


    junta.total_subjects += 1;

    Ok(())
}
