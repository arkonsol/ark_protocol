use anchor_lang::prelude::*;
use crate::states::{Proposal, ProposalStatus};
use the_ark_program::{InstructionContext, Decision};
use crate::errors::ErrorCode;
use standard::cpi::accounts::RouteInstruction as RouterAccounts;
use standard::GovernmentInstruction;
use standard::program::Standard;

pub fn send_proposal_decision_to_router(
    ctx: Context<SendProposalDecision>,
    instruction_data: Vec<u8>
) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    let clock = &ctx.accounts.clock;

    // Check if the proposal is still active and voting period has ended
    if proposal.status != ProposalStatus::Active || clock.unix_timestamp < proposal.end_time {
        return Err(ErrorCode::ProposalNotActive.into());
    }

    // Determine the decision based on the existing vote counts
    let total_votes = proposal.for_votes + proposal.against_votes;
    let decision = if total_votes > 0 && proposal.for_votes > proposal.against_votes {
        Decision::Approve
    } else {
        Decision::Reject
    };

    // Prepare the accounts for the router CPI
    let cpi_accounts = RouterAccounts {
        router_state: ctx.accounts.router_state.to_account_info(),
        government_account: ctx.accounts.government_account.to_account_info(),
        decision_account: ctx.accounts.decision_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    // Prepare the CPI context
    let cpi_program = ctx.accounts.router_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // Prepare the instruction context
    let instruction_context = InstructionContext {
        program_id: ctx.accounts.proposal.key(),
        instruction_data,
        signer: ctx.accounts.authority.key(),
        accounts: ctx.remaining_accounts.iter().map(|a| *a.key).collect(),
        block_time: clock.unix_timestamp,
        instruction_index: 0, // You might want to track this separately
    };

    // Serialize the instruction context
    let instruction_context_data = instruction_context.try_to_vec()?;

    // Prepare the instruction data for the router program
    let mut router_ix_data = GovernmentInstruction::MakeDecision.try_to_vec()?;
    router_ix_data.extend_from_slice(&instruction_context_data);

    // Make the CPI call to the router's route_instruction
    standard::cpi::route_instruction(cpi_ctx, router_ix_data)?;

    // Update the proposal status
    let proposal = &mut ctx.accounts.proposal;
    proposal.status = if decision == Decision::Approve { 
        ProposalStatus::Passed 
    } else { 
        ProposalStatus::Rejected 
    };

    // Emit an event with the decision details
    emit!(ProposalDecisionEvent {
        proposal_id: proposal.id,
        decision,
        for_votes: proposal.for_votes,
        against_votes: proposal.against_votes,
        total_votes,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SendProposalDecision<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: This is the router program
    pub router_program: Program<'info, Standard>,
    /// CHECK: This account is checked in the CPI call
    pub router_state: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the CPI call
    pub government_account: UncheckedAccount<'info>,
    /// CHECK: This account is used to store the decision
    pub decision_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[event]
pub struct ProposalDecisionEvent {
    pub proposal_id: u64,
    pub decision: Decision,
    pub for_votes: u64,
    pub against_votes: u64,
    pub total_votes: u64,
}