use anchor_lang::prelude::*;
use crate::states::junta::Junta;
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct ImposeMartialLaw<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(mut)]
    pub leader: Signer<'info>,
}

pub fn martial_law(ctx: Context<ImposeMartialLaw>) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    require!(junta.leader == ctx.accounts.leader.key(), ErrorCode::Unauthorized);

    junta.martial_law_active = true;
    junta.dissent_level = junta.dissent_level.saturating_sub(20);

    Ok(())
}