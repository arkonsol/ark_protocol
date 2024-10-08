use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct Citizen {
    pub governance_pool: Pubkey,
    pub user: Pubkey,
    pub assemblies: Vec<Pubkey>,
    pub staked_tokens: u64,
    pub completed_tasks: u64,
    pub voting_power: u64,
    pub delegated_power: u64,
    pub joined_at: i64,
    pub bump: u8,
}

impl Citizen {
    pub const SPACE: usize = 8 + // discriminator
    32 + // governance_pool
    32 + // user
    4 + (32 * 10) + // assemblies (assuming max 10 assemblies)
    8 + // staked_tokens
    8 + // completed_tasks
    8 + // voting_power
    8 + // delegated_power
    8 + // joined_at
    1;  // bump

    pub fn calculate_voting_power(&mut self) -> Result<()> {
        let base_power = self.assemblies.len() as u64 * BASE_ASSEMBLY_POWER;
        let staked_power = self.staked_tokens * STAKE_POWER_MULTIPLIER;
        let task_power = self.completed_tasks * TASK_POWER_MULTIPLIER;
        self.voting_power = base_power + staked_power + task_power;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VotingPowerAction {
    JoinAssembly,
    CompleteTask { task_id: u64 },
    StakeTokens { amount: u64 },
    DelegatePower { amount: u64 },
}