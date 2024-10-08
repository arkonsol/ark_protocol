use anchor_lang::prelude::*;
use crate::states::{junta::Junta, citizen::Citizen, rebel::Rebel};


#[derive(Accounts)]
#[instruction(rebellion_scale: u64)]
pub struct StartRebellion<'info> {
    #[account(mut)]
    pub junta: Box<Account<'info, Junta>>,

    #[account(mut)]
    pub rebel_leader: Signer<'info>,  

    #[account(
        init_if_needed,
        payer = rebel_leader,
        space = Rebel::SIZE, 
        seeds = [b"rebel", junta.key().as_ref(), &junta.total_subjects.to_le_bytes()],
        bump
    )]
    pub rebels: Box<Account<'info, Rebel>>,  

    #[account(
        init, 
        payer = rebel_leader, 
        space = Citizen::LEN, 
        seeds = [b"citizen", junta.key().as_ref(), &junta.total_subjects.to_le_bytes()],
        bump
    )]
    pub citizen: Box<Account<'info, Citizen>>, 

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn start_rebellion(ctx: Context<StartRebellion>) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let rebels = &mut ctx.accounts.rebels;
    let citizen = &mut ctx.accounts.citizen;

    // Initialize the Rebel account if it's new
    if rebels.count == 0 {
        rebels.rebels = [None; Rebel::MAX_REBELS];
    }

    // Check if the citizen can join the rebellion
    if !citizen.is_imprisoned {
        let citizen_data = Citizen {
            authority: citizen.authority,
            loyalty_score: citizen.loyalty_score,
            resources: citizen.resources,
            is_dissident: citizen.is_dissident,
            is_imprisoned: citizen.is_imprisoned,
            imprisonment_end: citizen.imprisonment_end,
            joined_at: citizen.joined_at,
            bump: ctx.bumps.citizen,
        };

        rebels.add_rebel(citizen_data)?;

        // Update junta state
        junta.dissent_level = junta.dissent_level.saturating_add(1);
        junta.control_level = junta.control_level.saturating_sub(1);

        // Update citizen state
        citizen.loyalty_score = citizen.loyalty_score.saturating_sub(5);
        citizen.is_dissident = true;

        msg!("Citizen joined the rebellion. Dissent increased.");
    } else {
        msg!("Citizen unable to join the rebellion.");
    }

    // Check if the rebellion succeeds
    if junta.control_level <= 0 {
        junta.is_overthrown = true;
        msg!("The Junta has been overthrown by the rebellion!");
    } else {
        msg!("The rebellion continues. Junta control: {}", junta.control_level);
    }

    Ok(())
}

