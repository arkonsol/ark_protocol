use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, Mint}, associated_token::AssociatedToken};
use the_ark_program::cpi::accounts::{RegisterGovernment, AddTokenToTreasury, CreateTreasury};
use the_ark_program::program::TheArkProgram;
use the_ark_program::state::analytics::ArkAnalytics;
use the_ark_program::cpi::register_government;
use the_ark_program::instructions::register_state::StateInfo;
use the_ark_program::instructions::register_state::GovernmentType;


pub mod error;

pub mod contexts;
pub mod states;

pub use contexts::*;
pub use states::*;

declare_id!("BTE5KLhjYqg6dhmUakXMNJkgYcsNiia5sQyp6wb1neqL");

#[program]
pub mod sortition {
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
        register_government(cpi_ctx, name, GovernmentType::Democracy)?;
    
        Ok(())
    }

    pub fn initialize_sortition_governance(ctx: Context<InitializeGovernance>, args: InitializeGovernmentArgs) -> Result<()> {
        initialize_governance(ctx, args)
    }

    pub fn register_sortition_citizen(ctx: Context<RegisterCitizen>, name: String, region: u8, age_group: u8, other_demographic: u8) -> Result<()> {
        register_citizen(ctx, name, region, age_group, other_demographic)
    }

    pub fn create_governance_invite(ctx: Context<CreateGovernanceInvite>, expiration_days: u64) -> Result<()> {
        create_invite::create_governance_invite(ctx, expiration_days)
    }

    pub fn use_governance_invite(ctx: Context<UseGovernanceInvite>, name: String, region: u8, age_group: u8, other_demographic: u8) -> Result<()> {
        use_invite::use_governance_invite(ctx, name, region, age_group, other_demographic)
    }

    pub fn select_sortition_assembly(ctx: Context<SelectAssembly>, term_length: i64) -> Result<()> {
        select_assembly(ctx, term_length)
    }

    pub fn finalize_sortition_assembly_selection<'info>(ctx: Context<'_, '_, 'info, 'info, FinalizeAssemblySelection<'info>>) -> Result<()> {
        finalize_assembly_selection(ctx)
    }

    pub fn create_new_proposal(ctx: Context<CreateProposal>, name: String, description: String, end_time: i64) -> Result<()> {
        create_proposal(ctx, name, description, end_time)
    }

    pub fn vote_on_sortition_proposal(ctx: Context<VoteOnProposal>, approve: bool) -> Result<()> {
        vote_on_proposal(ctx, approve)
    }

    pub fn mint_new_sbt(ctx: Context<MintSbt>, args: InitializeSbtArgs) -> Result<()> {
        mint_sbt(ctx, args)
    }

    pub fn mint_new_nft(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
        mint_nft(ctx, args)
    }

    pub fn initialize_spl_token(
        ctx: Context<InitializeToken>,
        params: InitTokenParams
    ) -> Result<()> {
        mint_spl::initialize_token(ctx, params)
    }

    pub fn mint_spl_tokens(ctx: Context<MintTokens>, amount_to_treasury: u64, amount_to_subject: u64) -> Result<()> {
        mint_spl::mint_tokens(ctx, amount_to_treasury, amount_to_subject)
    }

    pub fn add_new_governance_member(
        ctx: Context<AddGovernanceMember>, 
        name: String, 
        region: u8, 
        age_group: u8, 
        other_demographic: u8
    ) -> Result<()> {
        add_governance_member(ctx, name, region, age_group, other_demographic)
    }

    pub fn create_treasury(ctx: Context<CreateTreasuryCpi>, name: String) -> Result<()> {
        let cpi_program = ctx.accounts.treasury_program.to_account_info();
        let cpi_accounts = CreateTreasury {
            treasury: ctx.accounts.treasury.to_account_info(),
            owner: ctx.accounts.governance_pool.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        // Use the governance pool's key as the authority
        let authority = ctx.accounts.governance_pool.key();
        
        the_ark_program::cpi::create_government_treasury(cpi_ctx, name, authority)
    }

    pub fn add_token_to_treasury(ctx: Context<AddTokenToTreasuryCpi>) -> Result<()> {
        let cpi_program = ctx.accounts.treasury_program.to_account_info();
        let cpi_accounts = AddTokenToTreasury {
            treasury: ctx.accounts.treasury.to_account_info(),
            token_account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            owner: ctx.accounts.governance_pool.to_account_info(),
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
    pub governance_pool: Account<'info, GovernancePool>,
    /// CHECK: This account is checked in the CPI call
    pub treasury: UncheckedAccount<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub treasury_program: Program<'info, TheArkProgram>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct AddTokenToTreasuryCpi<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,
    /// CHECK: This account is checked in the CPI call
    pub treasury: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the CPI call
    pub token_account: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub treasury_program: Program<'info, TheArkProgram>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
