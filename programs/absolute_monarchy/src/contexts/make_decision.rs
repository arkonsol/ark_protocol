use anchor_lang::prelude::*;
use standard::cpi::accounts::RouteInstruction as RouterAccounts;
use standard::program::Standard;
use crate::states::Kingdom;

// Add this to your existing instruction set
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MakeDecisionArgs {
    pub instruction_data: Vec<u8>,
}

#[derive(Accounts)]
pub struct MakeDecision<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,
    pub monarch: Signer<'info>,
    /// CHECK: This is the router program
    pub router_program: Program<'info, Standard>,
    /// CHECK: This account is checked in the CPI call
    pub router_state: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the CPI call
    pub government_account: UncheckedAccount<'info>,
    /// CHECK: This account is used to store the decision
    pub decision_account: UncheckedAccount<'info>,
}


pub fn make_decision(ctx: Context<MakeDecision>, args: MakeDecisionArgs) -> Result<()> {
        // Ensure only the monarch can make decisions
        if ctx.accounts.monarch.key() != ctx.accounts.kingdom.monarch {
            return Err(ProgramError::InvalidAccountData.into());
        }

        // Prepare the accounts for the router CPI
        let cpi_accounts = RouterAccounts {
            router_state: ctx.accounts.router_state.to_account_info(),
            government_account: ctx.accounts.government_account.to_account_info(),
            decision_account: ctx.accounts.decision_account.to_account_info(),
            authority: ctx.accounts.monarch.to_account_info(),
        };

        // Prepare the CPI context
        let cpi_program = ctx.accounts.router_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Make the CPI call to the router's route_instruction
        standard::cpi::route_instruction(cpi_ctx, args.instruction_data)?;

        // The decision has been made and executed by the router

        Ok(())
}