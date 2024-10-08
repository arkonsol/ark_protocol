use anchor_lang::prelude::*;
use crate::states::Member;
use crate::errors::GovernanceError;

#[derive(Accounts)]
pub struct InitializeMember<'info> {
    #[account(
        init,
        payer = payer,
        space = Member::SPACE,
        seeds = [b"member", payer.key().as_ref()],
        bump
    )]
    pub member: Account<'info, Member>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_member(ctx: Context<InitializeMember>, name: String) -> Result<()> {
    require!(name.len() <= Member::MAX_NAME_LENGTH, GovernanceError::NameTooLong);

    let member = &mut ctx.accounts.member;
    member.name = name;
    member.circles = Vec::new();
    member.bump = ctx.bumps.member;

    Ok(())
}