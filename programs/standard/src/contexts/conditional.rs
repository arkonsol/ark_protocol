use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::escrow::{Escrow, EscrowData};
use crate::errors::RouterError;

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(
        init_if_needed,
        payer = sender,
        space = 8 + 32 + 32 + 32 + 8 + 200 + 1 + 8 + 1,
        seeds = [b"escrow", sender.key().as_ref(), recipient.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: This account is not written to or read from
    pub recipient: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = sender_token_account.owner == sender.key(),
        constraint = sender_token_account.mint == mint.key()
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = sender,
        associated_token::mint = mint,
        associated_token::authority = escrow,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"escrow_list"],
        bump = escrow_list.bump,
    )]
    pub escrow_list: Box<Account<'info, EscrowList>>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeEscrowList<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 4 + 10 * (32 + 32 + 32 + 8 + 200 + 1 + 8) + 1,
        seeds = [b"escrow_list"],
        bump
    )]
    pub escrow_list: Box<Account<'info, EscrowList>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePaymentList<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 4 + 10 * (8 + 32 + 8) + 1,
        seeds = [b"payment_list"],
        bump
    )]
    pub payment_list: Box<Account<'info, PaymentList>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FulfillCondition<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    pub recipient: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReleasePayment<'info> {
    #[account(
        mut,
        seeds = [b"escrow", sender.key().as_ref(), recipient_token_account.key().as_ref(), mint.key().as_ref()],
        bump = escrow.bump, 
        close = sender
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub sender: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = escrow_token_account.owner == escrow.key(),
        constraint = escrow_token_account.mint == mint.key()
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"payment_list"],
        bump = payment_list.bump,
    )]
    pub payment_list: Box<Account<'info, PaymentList>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut, has_one = sender, close = sender)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ListConditionalEscrows<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"escrow_list"],
        bump = escrow_list.bump,
    )]
    pub escrow_list: Box<Account<'info, EscrowList>>,
}

#[derive(Accounts)]
pub struct ListReleasedPayments<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"payment_list"],
        bump = payment_list.bump,
    )]
    pub payment_list: Box<Account<'info, PaymentList>>,
}

#[account]
pub struct EscrowList {
    pub escrows: Vec<EscrowData>,
    pub bump: u8,
}

#[account]
pub struct PaymentList {
    pub payments: Vec<Payment>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Payment {
    pub amount: u64,
    pub recipient: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct EscrowsListed {
    pub escrows: Vec<EscrowData>,
}

#[event]
pub struct PaymentsListed {
    pub payments: Vec<Payment>,
}

pub fn create_escrow(
    ctx: Context<CreateEscrow>,
    amount: u64,
    condition: String,
    expiry_time: i64,
) -> Result<()> {
    // Initialize escrow account
    ctx.accounts.escrow.set_inner(Escrow {
        sender: ctx.accounts.sender.key(),
        recipient: ctx.accounts.recipient.key(),
        mint: ctx.accounts.mint.key(),
        amount,
        condition: condition.clone(),
        is_fulfilled: false,
        expiry_time,
        bump: ctx.bumps.escrow,
    });

    let escrow_data = EscrowData {
        sender: ctx.accounts.sender.key(),
        recipient: ctx.accounts.recipient.key(),
        mint: ctx.accounts.mint.key(),
        amount,
        condition,
        is_fulfilled: false,
        expiry_time,
    };

    ctx.accounts.escrow_list.escrows.push(escrow_data);

    // Transfer tokens from sender to escrow account

    let cpi_accounts = Transfer {
        from: ctx.accounts.sender_token_account.to_account_info(),
        to: ctx.accounts.escrow_token_account.to_account_info(),
        authority: ctx.accounts.sender.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;


    Ok(())
}

pub fn fulfill_condition(ctx: Context<FulfillCondition>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    require!(!escrow.is_fulfilled, RouterError::AlreadyFulfilled);
    require!(
        Clock::get()?.unix_timestamp <= escrow.expiry_time,
        RouterError::Expired
    );

    escrow.is_fulfilled = true;
    Ok(())
}

pub fn release_payment(ctx: Context<ReleasePayment>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;
    require!(escrow.is_fulfilled, RouterError::ConditionNotFulfilled);
    require!(
        Clock::get()?.unix_timestamp <= escrow.expiry_time,
        RouterError::Expired
    );

    let payment = Payment {
        amount: ctx.accounts.escrow.amount,
        recipient: ctx.accounts.recipient_token_account.key(),
        timestamp: Clock::get()?.unix_timestamp,
    };

    ctx.accounts.payment_list.payments.push(payment);

    let sender = ctx.accounts.sender.key();
    let recipient = ctx.accounts.recipient_token_account.key();
    let mint = ctx.accounts.mint.key();

    let escrow_seeds = &[
        b"escrow",
        sender.as_ref(),
        recipient.as_ref(),
        mint.as_ref(),
        &[escrow.bump],
    ];
    let signer = &[&escrow_seeds[..]];

    // Transfer tokens from escrow to recipient
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, escrow.amount)?;



    Ok(())
}

pub fn refund(ctx: Context<Refund>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;
    require!(!escrow.is_fulfilled, RouterError::AlreadyFulfilled);
    require!(
        Clock::get()?.unix_timestamp > escrow.expiry_time,
        RouterError::NotExpired
    );

    // Transfer tokens from escrow back to sender
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        to: ctx.accounts.sender_token_account.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, escrow.amount)?;

    Ok(())
}

pub fn list_conditional_escrows(ctx: Context<ListConditionalEscrows>) -> Result<()> {
    let escrows = &ctx.accounts.escrow_list.escrows;
    
    // Filter out fulfilled escrows
    let active_escrows: Vec<EscrowData> = escrows
        .iter()
        .filter(|e| !e.is_fulfilled)
        .cloned()
        .collect();

    msg!("Listing conditional escrows...");
    for escrow in &active_escrows {
        msg!("Escrow: {:?}", escrow);
    }

    emit!(EscrowsListed {
        escrows: active_escrows,
    });

    Ok(())
}

pub fn list_released_payments(ctx: Context<ListReleasedPayments>) -> Result<()> {
    let payments = &ctx.accounts.payment_list.payments;

    msg!("Listing released payments...");
    for payment in payments {
        msg!("Payment: {:?}", payment);
    }

    emit!(PaymentsListed {
        payments: payments.clone(),
    });

    Ok(())
}

pub fn initialize_escrow_list(ctx: Context<InitializeEscrowList>) -> Result<()> {
    let escrow_list = &mut ctx.accounts.escrow_list;
    escrow_list.escrows = Vec::new();
    escrow_list.bump = ctx.bumps.escrow_list;
    Ok(())
}

pub fn initialize_payment_list(ctx: Context<InitializePaymentList>) -> Result<()> {
    let payment_list = &mut ctx.accounts.payment_list;
    payment_list.payments = Vec::new();
    payment_list.bump = ctx.bumps.payment_list;
    Ok(())
}


