use anchor_lang::prelude::*;
use crate::states::junta::Junta;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct TransferLeadership<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(mut)]
    pub current_leader: Signer<'info>,
    #[account(mut)]
    /// CHECK: This account is not read or written, just used as a Pubkey
    pub new_leader: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn transfer_leadership(ctx: Context<TransferLeadership>, new_leader: Pubkey) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    require!(junta.leader == ctx.accounts.current_leader.key(), ErrorCode::Unauthorized);
    require!(new_leader == ctx.accounts.new_leader.key(), ErrorCode::InvalidLeader);

    junta.leader = new_leader;

    Ok(())
}