use anchor_lang::prelude::*;

pub mod state;
pub mod constants;
pub mod instructions;
pub mod utilities;
mod errors;
pub mod interface;

pub use instructions::*;
pub use state::*;
pub use utilities::*;
pub use interface::*;

declare_id!("9rkxTYZH7uF5kd3xt9yrbvMEUFbeJCkfwFzSeqhmkN76");

#[program]
pub mod the_ark_program {
    use super::*;

    use crate::errors::ErrorCode;



    pub fn initialize_ark(ctx: Context<InitializeArk>) -> Result<()> {
        let analytics = &mut ctx.accounts.ark_analytics;
        analytics.initialized_at = Clock::get()?.unix_timestamp;
        analytics.governments = Vec::new();
        analytics.total_governments = 0;
        analytics.bump = ctx.bumps.ark_analytics;
        Ok(())
    }

    pub fn register_government(ctx: Context<RegisterGovernment>, name: String, government_type: GovernmentType) -> Result<()> {
        require!(name.len() <= StateInfo::MAX_NAME_LENGTH, ErrorCode::NameTooLong);

        // pub name: String,
        // pub government_type: GovernmentType,
        // pub creator: Pubkey,
        // pub created_at: i64,
        // pub program_id: Pubkey,
        
        let state_info = &mut ctx.accounts.state_info;

            state_info.name = name;
            state_info.government_type = government_type;
            state_info.creator = ctx.accounts.payer.key();
            state_info.program_id = ctx.accounts.government_program.key();
            state_info.bump = ctx.bumps.state_info;


        // Update ArkAnalytics if needed
        let ark_analytics = &mut ctx.accounts.ark_analytics;
        ark_analytics.governments.push(ctx.accounts.government_program.key());
        ark_analytics.total_governments += 1;
        
        emit!(StateRegistered {
            name: state_info.name.clone(),
            government_type: state_info.government_type.clone(),
            program_id: state_info.program_id,
        });

        Ok(())
    }

    pub fn create_government_treasury(ctx: Context<CreateTreasury>, name: String, authority: Pubkey) -> Result<()> {
        create_treasury(ctx, name, authority)
    }

    pub fn add_new_token_to_treasury(ctx: Context<AddTokenToTreasury>) -> Result<()> {
        add_token_to_treasury(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
