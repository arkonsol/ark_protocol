use anchor_lang::prelude::*;
use crate::states::{GovernancePool, Task};

#[derive(Accounts)]
#[instruction(task_id: u64, description: String, reward: u64)]
pub struct CreateTask<'info> {
    #[account(
        mut,
        has_one = admin,
    )]
    pub governance_pool: Account<'info, GovernancePool>,

    #[account(
        init,
        payer = admin,
        space = 8 + 8 + 32 + 200 + 8 + 33 + 1, // Adjust space as needed
        seeds = [b"task", governance_pool.key().as_ref(), &task_id.to_le_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_task(ctx: Context<CreateTask>, task_id: u64, description: String, reward: u64) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let governance_pool = &mut ctx.accounts.governance_pool;

    task.id = task_id;
    task.governance_pool = governance_pool.key();
    task.description = description;
    task.reward = reward;
    task.completed_by = None;
    task.bump = ctx.bumps.task;

    governance_pool.total_tasks += 1;

    Ok(())
}