use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

use crate::states::junta::Junta;
use crate::states::citizen::Citizen;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct RewardLoyalty<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(mut)]
    pub leader: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub citizen: Account<'info, Citizen>,
    #[account(
        init_if_needed,
        payer = leader,
        associated_token::mint = mint,
        associated_token::authority = citizen,
        associated_token::token_program = token_program,
    )]
    pub citizen_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn reward_loyalty(ctx: Context<RewardLoyalty>, amount: u64) -> Result<()> {
    let junta = &ctx.accounts.junta;
    
    require!(
        junta.leader == ctx.accounts.leader.key(),
        ErrorCode::Unauthorized
    );

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.citizen_ata.to_account_info(),
        authority: ctx.accounts.leader.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::mint_to(cpi_context, amount)?;

    msg!("Recipent successfully rewarded!");

    Ok(())
}
