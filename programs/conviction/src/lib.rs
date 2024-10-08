use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, Mint}, associated_token::AssociatedToken};
use the_ark_program::cpi::accounts::{RegisterGovernment, AddTokenToTreasury, CreateTreasury};
use the_ark_program::program::TheArkProgram;
use the_ark_program::state::analytics::ArkAnalytics;
use the_ark_program::cpi::register_government;
use the_ark_program::instructions::register_state::StateInfo;
use the_ark_program::instructions::register_state::GovernmentType;


pub mod errors;

pub mod contexts;
pub mod states;

pub use contexts::*;
pub use states::*;

declare_id!("DnN713Fw9TFRiNfXL3uTNGyXUdktQr9VQsHnVRWLtc8p");

#[program]
pub mod conviction {
    use super::*;

    pub fn initialize_and_register_government(ctx: Context<Initialize>, name: String) -> Result<()> {
        // Create CPI context
        let cpi_program = ctx.accounts.ark_program.to_account_info();
        let cpi_accounts = RegisterGovernment {
            payer: ctx.accounts.creator.to_account_info(),
            ark_analytics: ctx.accounts.ark_analytics.to_account_info(),
            state_info: ctx.accounts.state_info.to_account_info(),
            government_program: ctx.accounts.government_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        // Make the CPI call
        register_government(cpi_ctx, name, GovernmentType::Oligarchy)?;
    
        Ok(())
    }

    pub fn cancel_conviction_proposal(ctx: Context<CancelProposal>) -> Result<()> {
        cancel_proposal(ctx)
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote: bool, voting_power: u64) -> Result<()> {
        vote_with_power(ctx, vote, voting_power)
    }

    pub fn create_conviction_invite(ctx: Context<CreateInvite>, expiration_days: u64) -> Result<()> {
        create_invite(ctx, expiration_days)
    }

    pub fn use_conviction_invite(ctx: Context<UseInvite>) -> Result<()> {
        use_invite(ctx)
    }

    pub fn new_proposal(
        ctx: Context<CreateProposal>,
        description: String,
        voting_period: i64,
        execution_delay: i64,
        proposal_type: ProposalType,
    ) -> Result<()> {
        create_proposal(ctx, description, voting_period, execution_delay, proposal_type)
    }

    pub fn send_decision_to_router(
        ctx: Context<SendProposalDecision>,
        instruction_data: Vec<u8>
    ) -> Result<()> {
        send_proposal_decision_to_router(ctx, instruction_data)
    }

    pub fn conclude_proposal(ctx: Context<EndAndExecuteProposal>) -> Result<()> {
        end_and_execute_proposal(ctx)
    }

    pub fn new_governance(
        ctx: Context<InitializeGovernance>,
        args: InitializeGovernanceArgs
    ) -> Result<()> {
        initialize_governance(ctx, args)
    }

    pub fn create_stake_account(ctx: Context<InitializeStakeAccount>) -> Result<()> {
        initialize_stake_account(ctx)
    }

    pub fn mint_conviction_nft(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
        mint_nft(ctx, args)
    }

    pub fn mint_conviction_sbt(ctx: Context<MintConvictionSbt>, args: InitializeSbtArgs) -> Result<()> {
        mint_sbt(ctx, args)
    }

    pub fn init_token(
        ctx: Context<InitializeToken>,
        params: InitTokenParams
    ) -> Result<()> {
        initialize_token(ctx, params)
    }

    pub fn mint_conviction_tokens(ctx: Context<MintTokens>, amount_to_treasury: u64, amount_to_citizen: u64) -> Result<()> {
        mint_tokens(ctx, amount_to_treasury, amount_to_citizen)
    }

    pub fn stake_nft_on_proposal(ctx: Context<StakeNftOnProposal>, lock_period: u8) -> Result<()> {
        stake_nft(ctx, lock_period)
    }

    pub fn stake_spltokens_on_proposal(ctx: Context<StakeOnProposal>, amount: u64, lock_period: u8) -> Result<()> {
        stake(ctx, amount, lock_period)
    }

    pub fn add_new_member(ctx: Context<AddMember>) -> Result<()> {
        add_member(ctx)
    }

    pub fn unstake_nft_from_vault(ctx: Context<UnstakeNftFromProposal>) -> Result<()> {
        unstake_nft(ctx)
    }

    pub fn unstake_spltokens_from_vault(ctx: Context<UnstakeFromProposal>, amount: u64) -> Result<()> {
        unstake(ctx, amount)
    }

    pub fn create_treasury(ctx: Context<CreateTreasuryCpi>, name: String) -> Result<()> {
        let cpi_program = ctx.accounts.treasury_program.to_account_info();
        let cpi_accounts = CreateTreasury {
            treasury: ctx.accounts.treasury.to_account_info(),
            owner: ctx.accounts.governance.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        // Use the governance pool's key as the authority
        let authority = ctx.accounts.governance.key();
        
        the_ark_program::cpi::create_government_treasury(cpi_ctx, name, authority)
    }

    pub fn add_token_to_treasury(ctx: Context<AddTokenToTreasuryCpi>) -> Result<()> {
        let cpi_program = ctx.accounts.treasury_program.to_account_info();
        let cpi_accounts = AddTokenToTreasury {
            treasury: ctx.accounts.treasury.to_account_info(),
            token_account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            owner: ctx.accounts.governance.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        the_ark_program::cpi::add_new_token_to_treasury(cpi_ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub ark_analytics: Account<'info, ArkAnalytics>,
    #[account(mut)]
    pub state_info: Account<'info, StateInfo>,
    /// CHECK: This is the program ID of the specific government type
    pub government_program: UncheckedAccount<'info>,
    pub ark_program: Program<'info, TheArkProgram>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateTreasuryCpi<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,
    /// CHECK: This account is checked in the CPI call
    pub treasury: UncheckedAccount<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub treasury_program: Program<'info, TheArkProgram>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct AddTokenToTreasuryCpi<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,
    /// CHECK: This account is checked in the CPI call
    pub treasury: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the CPI call
    pub token_account: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub treasury_program: Program<'info, TheArkProgram>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
