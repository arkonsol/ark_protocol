use anchor_lang::{prelude::*, system_program};
use crate::states::junta::Junta;
use crate::errors::ErrorCode;


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ResourceAction {
    Collect,
    Distribute,
}

#[derive(Accounts)]
pub struct ManageResources<'info> {
    #[account(mut, seeds = [b"junta", junta.name.as_bytes()], bump = junta.bump)]
    pub junta: Account<'info, Junta>,

    #[account(mut)]
    pub recipient: SystemAccount<'info>,

    #[account(
        constraint = 
            (authority.key() == junta.leader || junta.officers.contains(&authority.key()))
            @ ErrorCode::Unauthorized
    )]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn manage_resources(ctx: Context<ManageResources>, action: ResourceAction, amount: u64) -> Result<()> {
    let junta = &mut ctx.accounts.junta;

    match action {
        ResourceAction::Collect => {
            // Transfer SOL from authority to junta PDA
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.authority.to_account_info(),
                        to: junta.to_account_info(),
                    },
                ),
                amount,
            )?;

            junta.resources = junta.resources.checked_add(amount)
                .ok_or(ErrorCode::Overflow)?;
        },
        ResourceAction::Distribute => {
            require!(junta.resources >= amount, ErrorCode::InsufficientResources);

            // Transfer SOL from junta PDA to recipient
            let seeds = &[
                b"junta",
                junta.name.as_bytes(),
                &[junta.bump],
            ];
            let signer = &[&seeds[..]];

            system_program::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: junta.to_account_info(),
                        to: ctx.accounts.recipient.to_account_info(),
                    },
                    signer,
                ),
                amount,
            )?;

            junta.resources = junta.resources.checked_sub(amount)
                .ok_or(ErrorCode::Underflow)?;
        },
    }

    emit!(ResourceManaged {
        action: action,
        amount: amount,
        new_balance: junta.resources,
        authority: ctx.accounts.authority.key(),
        recipient: ctx.accounts.recipient.key(),
    });

    Ok(())
}

#[event]
pub struct ResourceManaged {
    pub action: ResourceAction,
    pub amount: u64,
    pub new_balance: u64,
    pub authority: Pubkey,
    pub recipient: Pubkey,
}