use anchor_lang::prelude::*;
use crate::states::{junta::Junta, citizen::Citizen};
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct SuppressDissent<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(mut)]
    pub citizen: Account<'info, Citizen>,
    #[account(mut)]
    pub suppressor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn suppress_dissent(ctx: Context<SuppressDissent>, target: Pubkey) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let citizen = &mut ctx.accounts.citizen;

    require!(
        junta.leader == ctx.accounts.suppressor.key() || junta.officers.contains(&ctx.accounts.suppressor.key()),
        ErrorCode::Unauthorized
    );
    require!(citizen.authority == target, ErrorCode::InvalidTarget);

    if citizen.is_dissident {
        citizen.loyalty_score = citizen.loyalty_score.saturating_sub(10);
        citizen.is_dissident = false;
        junta.dissent_level = junta.dissent_level.saturating_sub(1);
    }

    Ok(())
}