use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, Mint}, associated_token::AssociatedToken};
use the_ark_program::cpi::accounts::{RegisterGovernment, AddTokenToTreasury, CreateTreasury};
use the_ark_program::program::TheArkProgram;
use the_ark_program::state::analytics::ArkAnalytics;
use the_ark_program::cpi::register_government;
use the_ark_program::instructions::register_state::StateInfo;
use the_ark_program::instructions::register_state::GovernmentType;

pub mod states;
pub mod contexts;
mod errors;

pub use contexts::*;
pub use states::*;

declare_id!("6A9RQDic1z5DfneBEyu4N7aPXmGC4wGQVAEzUUdYybDY");

#[program]
pub mod military {
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
        register_government(cpi_ctx, name, GovernmentType::Autocracy)?;
    
        Ok(())
    }

    pub fn initialize_mil_junta(ctx: Context<InitializeJunta>, args: InitializeJuntaArgs) -> Result<()> {
        initialize_junta(ctx, args)
    }

    pub fn create_junta_invite(ctx: Context<CreateJuntaInvite>, expiration_days: u64) -> Result<()> {
        create_invite::create_junta_invite(ctx, expiration_days)
    }

    pub fn use_junta_invite(ctx: Context<UseJuntaInvite>) -> Result<()> {
        use_invite::use_junta_invite(ctx)
    }

    pub fn appoint_officers(ctx: Context<AppointOfficer>, rank: u8) -> Result<()> {
        appoint_officer(ctx, rank)
    }

    pub fn issue_decrees(ctx: Context<IssueDecree>, content: String) -> Result<()> {
        issue_decree(ctx, content)
    }

    pub fn suppress_dissents(ctx: Context<SuppressDissent>, target: Pubkey) -> Result<()> {
        suppress_dissent(ctx, target)
    }

    pub fn manage_junta_resources(ctx: Context<ManageResources>, action: ResourceAction, amount: u64) -> Result<()> {
        manage_resources(ctx, action, amount)
    }

    pub fn delegate_token_account(ctx: Context<ApproveDelegate>, amount: u64) -> Result<()> {
        approve_delegate(ctx, amount)
    }

    pub fn exile(ctx: Context<ExileCitizen>, target: Pubkey, amount_to_burn: u64) -> Result<()> {
        exile_citizen(ctx, target, amount_to_burn)
    }

    pub fn imprison(ctx: Context<ImprisonCitizen>, target: Pubkey, end_date: Option<i64>, amount_to_seize: u64) -> Result<()> {
        imprison_citizen(ctx, target, end_date, amount_to_seize)
    }

    pub fn init_citizen(ctx: Context<InitializeCitizen>, authority: Pubkey) -> Result<()> {
        initialize_citizen(ctx, authority)
    }

    pub fn impose_martiallaw(ctx: Context<ImposeMartialLaw>) -> Result<()> {
        martial_law(ctx)
    }

    pub fn start_rebel(ctx: Context<StartRebellion>) -> Result<()> {
        start_rebellion(ctx)
    }

    pub fn reward_citizen(ctx: Context<RewardLoyalty>, amount: u64) -> Result<()> {
        reward_loyalty(ctx, amount)
    }

    pub fn support_junta(ctx: Context<GainSupport>,  amount: u64) -> Result<()> {
        gain_support(ctx, amount)
    }

    pub fn change_leadership(ctx: Context<TransferLeadership>, new_leader: Pubkey) -> Result<()> {
        transfer_leadership(ctx, new_leader)
    }

    pub fn mint_nfts_to_citizen(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
        mint_nft(ctx, args)
    }

    pub fn initialize_spl_token(
        ctx: Context<InitializeToken>,
        params: InitTokenParams
    ) -> Result<()> {
        initialize_token(ctx, params)
    }

    pub fn add_new_junta_member(ctx: Context<AddJuntaMember>) -> Result<()> {
        add_junta_member(ctx)
    }

    pub fn mint_sbts_to_citizen(ctx: Context<MintJuntaSbt>, args: InitializeSbtArgs) -> Result<()> {
        mint_sbt(ctx, args)
    }

    pub fn mint_spl_tokens(ctx: Context<MintTokens>, amount_to_treasury: u64, amount_to_citizen: u64) -> Result<()> {
        mint_tokens(ctx,  amount_to_treasury, amount_to_citizen)
    }

    pub fn create_treasury(ctx: Context<CreateTreasuryCpi>, name: String) -> Result<()> {
        let cpi_program = ctx.accounts.treasury_program.to_account_info();
        let cpi_accounts = CreateTreasury {
            treasury: ctx.accounts.treasury.to_account_info(),
            owner: ctx.accounts.junta.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        // Use the governance pool's key as the authority
        let authority = ctx.accounts.junta.key();
        
        the_ark_program::cpi::create_government_treasury(cpi_ctx, name, authority)
    }

    pub fn add_token_to_treasury(ctx: Context<AddTokenToTreasuryCpi>) -> Result<()> {
        let cpi_program = ctx.accounts.treasury_program.to_account_info();
        let cpi_accounts = AddTokenToTreasury {
            treasury: ctx.accounts.treasury.to_account_info(),
            token_account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            owner: ctx.accounts.junta.to_account_info(),
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
    pub junta: Account<'info, Junta>,
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
    pub junta: Account<'info, Junta>,
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
