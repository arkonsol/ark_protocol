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
declare_id!("D2VDfq9f7UaeuJVqpR5Qq1W4vBfqHbJWYSbvbE1Bsryc");

#[program]
pub mod absolute_monarchy {
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
        register_government(cpi_ctx, name, GovernmentType::Monarchy)?;
    
        Ok(())
    }

    pub fn initialize_absolute_monarchy(ctx: Context<InitializeKingdom>, args: InitializeKingdomArgs) -> Result<()> {
        init_state::initialize_kingdom(ctx, args)
    }
    

    pub fn abdicate(ctx: Context<Abdicate>, heir_name: String) -> Result<()> {
        abdicate::abdicate(ctx, heir_name)
    }

    pub fn make_decision(ctx: Context<MakeDecision>, args: MakeDecisionArgs) -> Result<()> {
        make_decision::make_decision(ctx, args)
    }

    pub fn create_kingdom_invite(ctx: Context<CreateKingdomInvite>, expiration_days: u64) -> Result<()> {
        create_invite::create_kingdom_invite(ctx, expiration_days)
    }

    pub fn use_kingdom_invite(ctx: Context<UseKingdomInvite>) -> Result<()> {
        use_invite::use_kingdom_invite(ctx)
    }

    // Legislative Powers
    pub fn decree(ctx: Context<DecreeContext>, decree_text: String, decree_type: DecreeType) -> Result<()> {
        init_decree::decree(ctx, decree_text, decree_type)
    }

    pub fn repeal_decree(ctx: Context<RepealDecree>, decree_id: u64) -> Result<()> {
        init_decree::repeal_decree(ctx, decree_id)
    }

    pub fn appoint_official(ctx: Context<AppointOfficial>, role: String, jurisdiction: String) -> Result<()> {
        appoint::appoint_official(ctx, role, jurisdiction)
    }

    pub fn implement_policy(ctx: Context<ImplementPolicy>, title: String, description: String, target_jurisdiction: String) -> Result<()> {
        contexts::policy::implement_policy(ctx, title, description, target_jurisdiction)
    }

    pub fn update_policy(
        ctx: Context<UpdatePolicy>, 
        new_description: Option<String>, 
        new_target_jurisdiction: Option<String>,
        new_is_active: Option<bool>
    ) -> Result<()> {
        contexts::policy::update_policy(ctx, new_description, new_target_jurisdiction, new_is_active)
    }

    pub fn royal_judgment(ctx: Context<RoyalJudgment>, verdict: Verdict, punishment: Option<Punishment>) -> Result<()> {
       judicial::royal_judgment(ctx, verdict, punishment)
    }

    pub fn royal_pardon(ctx: Context<RoyalPardon>) -> Result<()> {
        judicial::royal_pardon(ctx)
    }

    pub fn declare_war(ctx: Context<DeclareWar>, reason: String) -> Result<()> {
        military::declare_war(ctx, reason)
    }

    pub fn set_economic_policy(
        ctx: Context<SetEconomicPolicy>, 
        policy: EconomicPolicyType, 
        income_tax_rate: u8,
        property_tax_rate: u8,
        trade_tax_rate: u8,
        luxury_tax_rate: u8
    ) -> Result<()> {
        contexts::economy::set_economic_policy(ctx, policy, income_tax_rate, property_tax_rate, trade_tax_rate, luxury_tax_rate)
    }

    pub fn pay_tax(ctx: Context<PayTax>, tax_type: TaxType, taxable_amount: u64) -> Result<()> {
        contexts::pay_tax(ctx, tax_type, taxable_amount)
    }

    pub fn grant_privileged_access(
        ctx: Context<GrantPrivilegedAccess>, 
        access_type: String, 
        duration: i64, 
        usage_fee_rate: u8,
        access_level: u8,
        initial_fee: u64
    ) -> Result<()> {
        contexts::grant_privileged_access(ctx, access_type, duration, usage_fee_rate, access_level, initial_fee)
    }

    pub fn unappoint_official(ctx: Context<UnappointOfficial>) -> Result<()> {
        contexts::unappoint_official(ctx)
    }

    pub fn add_member(ctx: Context<AddMember>) -> Result<()> {
        contexts::add_member(ctx)
    }

    pub fn use_privileged_access(ctx: Context<UsePrivilegedAccess>, usage_amount: u64) -> Result<()> {
        contexts::use_privileged_access(ctx, usage_amount)
    }

    // Social Structure
    pub fn grant_nobility(ctx: Context<GrantNobility>, title: String) -> Result<()> {
        contexts::status::grant_nobility(ctx, title)
    }

    pub fn expand_organization(ctx: Context<ExpandOrganization>, new_division_name: String, budget: u64) -> Result<()> {
        expansion::expand_organization(ctx, new_division_name, budget)
    }

    pub fn transfer_revenue(ctx: Context<TransferRevenue>) -> Result<()> {
        expansion::transfer_revenue(ctx)
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
        initialize_token(ctx, params)
    }

    pub fn mint_spl_tokens_to_kingdom(ctx: Context<MintToKingdom>, amount: u64) -> Result<()> {
        mint_to_kingdom(ctx, amount)
    }

    pub fn mint_spl_tokens_to_subject(ctx: Context<MintToSubject>, amount: u64) -> Result<()> {
        mint_to_subject(ctx, amount)
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
    pub governance: Account<'info, Kingdom>,
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
    pub governance: Account<'info, Kingdom>,
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
