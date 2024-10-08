// To register an instance created from any government state
use anchor_lang::prelude::*;
use crate::state::analytics::ArkAnalytics;

#[derive(Accounts)]
pub struct RegisterGovernment<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"ark_analytics"], bump)]
    pub ark_analytics: Account<'info, ArkAnalytics>,
    #[account(
        init,
        payer = payer,
        space = StateInfo::LEN,
        seeds = [b"state_info"],
        bump
    )]
    pub state_info: Account<'info, StateInfo>,
    #[account(mut)]
    /// CHECK: This is the program ID of the specific government type
    pub government_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StateInfo {
    pub name: String,
    pub government_type: GovernmentType,
    pub creator: Pubkey,
    pub created_at: i64,
    pub program_id: Pubkey,
    pub bump: u8,
}

impl StateInfo {
    pub const MAX_NAME_LENGTH: usize = 50; // Adjust this value as needed

    pub const LEN: usize = 8 + // discriminator
        4 + Self::MAX_NAME_LENGTH + // name (String)
        1 + // government_type (assuming it's an enum with a small number of variants)
        32 + // creator (Pubkey)
        8 + // created_at (i64)
        32 + // program_id (Pubkey)
        1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum GovernmentType {
    Monarchy,
    Democracy,
    Autocracy,
    Oligarchy,
    Republic,
    Federalism,
    Communism,
    Imperialism,
    Anarchy,
    Colonialism
}

impl Default for GovernmentType {
    fn default() -> Self {
        GovernmentType::Monarchy 
    }
}

#[event]
pub struct StateRegistered {
    pub name: String,
    pub government_type: GovernmentType,
    pub program_id: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum GlobalEvent {
    NaturalDisaster,
    EconomicBoom,
    Pandemic,
    // Add more as needed
}

#[account]
pub struct StateAccount {
    pub name: String,
    pub government_type: GovernmentType,
    pub stability: u8,
    pub at_war: bool,
    // Add other fields as necessary
}
