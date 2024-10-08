use anchor_lang::prelude::*;
use crate::states::{circle::Circle, member::Member};
use crate::errors::GovernanceError;

#[derive(Accounts)]
pub struct ElectMember<'info> {
    #[account(mut)]
    pub circle: Account<'info, Circle>,
    #[account(
        init_if_needed,
        payer = payer,
        space = Member::SPACE,
        seeds = [b"member", member.key().as_ref()],
        bump
    )]
    pub member: Box<Account<'info, Member>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn elect_member(ctx: Context<ElectMember>, name: String) -> Result<()> {
    let circle = &mut ctx.accounts.circle;
    let member = &mut ctx.accounts.member;

    if circle.members.contains(&member.key()) {
        return Err(GovernanceError::MemberAlreadyExists.into());
    }

    circle.members.push(member.key());
    if !member.circles.contains(&circle.key()) {
        member.circles.push(circle.key());
    }
    member.name = name;

    Ok(())
}