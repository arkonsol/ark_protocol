use anchor_lang::prelude::*;
use crate::states::{junta::Junta, decree::Decree};
use crate::errors::ErrorCode;


#[derive(Accounts)]
#[instruction(content: String)]
pub struct IssueDecree<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(
        init,
        payer = issuer,
        space = 8 + 32 + Decree::MAX_CONTENT_LENGTH + 8 + 1,
        seeds = [b"decree", junta.key().as_ref(), &junta.total_subjects.to_le_bytes()],
        bump
    )]
    pub decree: Account<'info, Decree>,
    #[account(mut)]
    pub issuer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn issue_decree(ctx: Context<IssueDecree>, content: String) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let decree = &mut ctx.accounts.decree;

    require!(
        junta.leader == ctx.accounts.issuer.key() || junta.officers.contains(&ctx.accounts.issuer.key()),
        ErrorCode::Unauthorized
    );
    
    require!(junta.decrees.len() < Junta::MAX_DECREES, ErrorCode::TooManyDecrees);

    decree.issuer = ctx.accounts.issuer.key();
    decree.content = content;
    decree.issued_at = Clock::get()?.unix_timestamp;
    decree.bump = ctx.bumps.decree;

    junta.decrees.push(decree.key());

    Ok(())
}