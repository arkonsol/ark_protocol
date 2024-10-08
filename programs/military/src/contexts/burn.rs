// use anchor_lang::prelude::*;
// use anchor_spl::token::{self, TokenAccount, Burn, Mint, Token};

// use crate::states::{junta::Junta, citizen::Citizen};

// #[derive(Accounts)]
// pub struct BurnGovernanceTokens<'info> {
//     #[account(mut)]
//     pub junta: Account<'info, Junta>,  
//     #[account(mut)]
//     pub citizen: Account<'info, Citizen>,
//     #[account(mut)]
//     pub citizen_governance_tokens: Account<'info, TokenAccount>, 
//     #[account(mut)]
//     pub governance_token_mint: Account<'info, Mint>, 
//     #[account(signer)]
//     pub leader: Signer<'info>,
//     pub token_program: Program<'info, Token>,  
// }

// pub fn burn_governance_tokens(ctx: Context<BurnGovernanceTokens>, amount: u64) -> Result<()> {
//     let junta = &ctx.accounts.junta;

//     require!(junta.leader == ctx.accounts.leader.key(), ErrorCode::Unauthorized);
//     let cpi_accounts = Burn {
//         mint: ctx.accounts.governance_token_mint.to_account_info(),
//         from: ctx.accounts.citizen_governance_tokens.to_account_info(),
//         authority: ctx.accounts.leader.to_account_info(),
//     };

//     let cpi_program = ctx.accounts.token_program.to_account_info();
//     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

//     token::burn(cpi_ctx, amount)?;

//     Ok(())
// }

