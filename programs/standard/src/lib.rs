use anchor_lang::prelude::*;
use the_ark_program::{GovernmentTypes, InstructionContext, Decision};
use borsh::{BorshSerialize, BorshDeserialize};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

pub mod contexts;
pub use contexts::*;
mod errors;

pub mod states;
pub use states::*;

declare_id!("2eTVXGSKCaTaivadMawTEz3h14FLwhyGTY64UBYLyZ6p");

#[derive(BorshSerialize, BorshDeserialize)]
pub enum GovernmentInstruction {
    MakeDecision,
}

#[program]
pub mod standard {
    use super::*;

    use crate::errors::RouterError;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let router_state = &mut ctx.accounts.router_state;
        router_state.authority = ctx.accounts.authority.key();
        router_state.bump = ctx.bumps.router_state;
        Ok(())
    }

    pub fn register_government(
        ctx: Context<RegisterGovernment>, 
        government_type: GovernmentTypes,
        government_program_id: Pubkey
    ) -> Result<()> {
        let router_state = &mut ctx.accounts.router_state;
        
        if router_state.governments.len() >= 10 {
            return Err(RouterError::TooManyGovernments.into());
        }

        router_state.governments.push(GovernmentEntry {
            government_type,
            program_id: government_program_id,
        });

        Ok(())
    }

    pub fn route_instruction(
        ctx: Context<RouteInstruction>,
        instruction_data: Vec<u8>
    ) -> Result<()> {
        let router_state = &ctx.accounts.router_state;
        let government_account = &ctx.accounts.government_account;
        let decision_account = &ctx.accounts.decision_account;
    
        // Find the registered government program
        let government_entry = router_state.governments
            .iter()
            .find(|g| g.program_id == government_account.key())
            .ok_or(RouterError::GovernmentNotFound)?;
    
        // Create the instruction context
        let instruction_context = InstructionContext {
            program_id: government_entry.program_id,
            instruction_data: instruction_data.clone(),
            signer: ctx.accounts.authority.key(),
            accounts: ctx.remaining_accounts.iter().map(|a| *a.key).collect(),
            block_time: Clock::get()?.unix_timestamp,
            instruction_index: 0,
        };
    
        // Serialize the instruction context
        let mut instruction_context_data = Vec::new();
        instruction_context.serialize(&mut instruction_context_data)?;
    
        // Prepare the instruction data for the government program
        // let mut government_ix_data = Vec::new();
        // GovernmentInstruction::MakeDecision.serialize(&mut government_ix_data)?;
        // government_ix_data.extend_from_slice(&instruction_context_data);

        // let mut government_ix_data = Vec::new();
        // government_ix_data.push(0u8); 
        // government_ix_data.extend_from_slice(&instruction_context_data);

        let mut government_ix_data = GovernmentInstruction::MakeDecision.try_to_vec()?;
        government_ix_data.extend_from_slice(&instruction_context_data);
    
        // Call the government program to make a decision
        let make_decision_ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: government_entry.program_id,
            accounts: vec![
                AccountMeta::new(*government_account.key, false),
                AccountMeta::new(*decision_account.key, false),
                AccountMeta::new(ctx.accounts.authority.key(), true),
            ],
            data: government_ix_data,
        };
    
        let account_infos = &[
            government_account.to_account_info(),
            decision_account.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ];
    
        anchor_lang::solana_program::program::invoke(
            &make_decision_ix,
            account_infos,
        )?;

        // Read the decision from the decision account
        let decision = Decision::try_from_slice(&decision_account.data.borrow())?;

        match decision {
            Decision::Approve => {
                msg!("Decision approved by the government");
                // Execute the actual instruction
                let target_ix = anchor_lang::solana_program::instruction::Instruction {
                    program_id: instruction_context.program_id,
                    accounts: ctx.remaining_accounts.iter().map(|a| {
                        AccountMeta::new(a.key(), a.is_signer)
                    }).collect(),
                    data: instruction_data,
                };
                anchor_lang::solana_program::program::invoke(&target_ix, ctx.remaining_accounts)?;
            }
            Decision::Reject => {
                msg!("Decision rejected by the government");
                return Err(RouterError::DecisionRejected.into());
            }
        }

        Ok(())
    }

    pub fn initialize_market(ctx: Context<InitializeMarket>, base_mint: Pubkey, quote_mint: Pubkey) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.base_mint = base_mint;
        market.quote_mint = quote_mint;
        market.next_order_id = 1;
        market.bump = ctx.bumps.market;
        Ok(())
    }

    pub fn place_order(
        ctx: Context<PlaceOrder>,
        side: OrderSide,
        amount: u64,
        price: u64,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market;
        
        let order = Order {
            id: market.next_order_id,
            owner: ctx.accounts.owner.key(),
            side,
            amount,
            price,
        };
    
        market.orders.push(order);
        market.next_order_id += 1;
    
        let transfer_amount = if side == OrderSide::Bid {
            amount.checked_mul(price).ok_or(ProgramError::ArithmeticOverflow)?
        } else {
            amount
        };
    
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.owner_token_account.to_account_info(),
                    to: ctx.accounts.escrow_token_account.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            transfer_amount,
        )?;
    
        Ok(())
    }
    
    pub fn process_orders(ctx: Context<ProcessOrders>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        
        // Sort orders by price (descending for bids, ascending for asks)
        market.orders.sort_by(|a, b| {
            match a.side {
                OrderSide::Bid => b.price.cmp(&a.price),
                OrderSide::Ask => a.price.cmp(&b.price),
            }
        });

        let mut i = 0;
        while i < market.orders.len() {
            let order = &market.orders[i];
            let matching_order_index = market.orders.iter().skip(i + 1).position(|o| 
                o.side != order.side && 
                (order.side == OrderSide::Bid && order.price >= o.price ||
                 order.side == OrderSide::Ask && order.price <= o.price)
            );

            if let Some(j) = matching_order_index {
                let (order1, order2) = if i < j + i + 1 {
                    (&market.orders[i], &market.orders[j + i + 1])
                } else {
                    (&market.orders[j + i + 1], &market.orders[i])
                };

                let trade_amount = order1.amount.min(order2.amount);
                let trade_price = order2.price; // Use the ask price as the trade price

                // Execute trade
                execute_trade(
                    trade_amount,
                    trade_price,
                    &ctx.accounts.base_vault,
                    &ctx.accounts.quote_vault,
                    &ctx.accounts.token_program,
                )?;

                // Update order amounts
                market.orders[i].amount -= trade_amount;
                market.orders[j + i + 1].amount -= trade_amount;

                // Remove filled orders
                market.orders.retain(|o| o.amount > 0);
            } else {
                i += 1;
            }
        }
    
        Ok(())
    }

    pub fn list_placed_orders(ctx: Context<ListPlacedOrders>) -> Result<()> {
        let market = &ctx.accounts.market;
        
        msg!("Listing placed orders for market: {:?}", market.key());
        for order in &market.orders {
            msg!("Order ID: {}, Side: {:?}, Amount: {}, Price: {}", 
                order.id, order.side, order.amount, order.price);
        }
        
        Ok(())
    }

    pub fn create_conditional_escrow(
        ctx: Context<CreateEscrow>,
        amount: u64,
        condition: String,
        expiry_time: i64,
    ) -> Result<()> {
        create_escrow(ctx, amount, condition, expiry_time)
    }

    pub fn fulfill_payment_condition(ctx: Context<FulfillCondition>) -> Result<()> {
        fulfill_condition(ctx)
    }

    pub fn release_payment_for_condition(ctx: Context<ReleasePayment>) -> Result<()> {
        release_payment(ctx)
    }

    pub fn refund_payment(ctx: Context<Refund>) -> Result<()> {
        refund(ctx)
    }

    pub fn list_conditional_escrows(ctx: Context<ListConditionalEscrows>) -> Result<()> {
        conditional::list_conditional_escrows(ctx)
    }


    pub fn list_released_payments(ctx: Context<ListReleasedPayments>) -> Result<()> {
        conditional::list_released_payments(ctx)
    }

    pub fn initialize_escrow_list(ctx: Context<InitializeEscrowList>) -> Result<()> {
        conditional::initialize_escrow_list(ctx)
    }

    pub fn initialize_payment_list(ctx: Context<InitializePaymentList>) -> Result<()> {
        conditional::initialize_payment_list(ctx)
    }

    pub fn reclaim_verify(ctx: Context<Verify>, args: VerifyArgs) -> Result<()> {
        verify(ctx, args)
    }

    pub fn revoke_reclaim_verification(ctx: Context<RevokeVerification>) -> Result<()> {
    revoke_verification(ctx)
    }

    pub fn list_reclaim_verifications(ctx: Context<ListVerifications>) -> Result<()> {
        list_verifications(ctx)
    }
}

fn execute_trade<'info>(
    amount: u64,
    price: u64,
    base_vault: &Account<'info, TokenAccount>,
    quote_vault: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
) -> Result<()> {
    let base_amount = amount;
    let quote_amount = amount.checked_mul(price).ok_or(ProgramError::ArithmeticOverflow)?;

    // Transfer base tokens
    token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: base_vault.to_account_info(),
                to: quote_vault.to_account_info(),
                authority: base_vault.to_account_info(),
            },
        ),
        base_amount,
    )?;

    // Transfer quote tokens
    token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: quote_vault.to_account_info(),
                to: base_vault.to_account_info(),
                authority: quote_vault.to_account_info(),
            },
        ),
        quote_amount,
    )?;

    Ok(())
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 10 * (32 + 1) + 1, 
        seeds = [b"router_state"],
        bump
    )]
    pub router_state: Account<'info, RouterState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterGovernment<'info> {
    #[account(mut, has_one = authority)]
    pub router_state: Account<'info, RouterState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct RouteInstruction<'info> {
    #[account(mut)]
    pub router_state: Account<'info, RouterState>,
    /// CHECK: This account is not written to, and is checked in the instruction handler
    pub government_account: UncheckedAccount<'info>,
    /// CHECK: This account is used to store the decision from the government program
    pub decision_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8 + 1000 + 1, // Added 1 for bump
        seeds = [b"market", base_mint.key().as_ref(), quote_mint.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: This account is not written to or read from
    pub base_mint: UncheckedAccount<'info>,
    /// CHECK: This account is not written to or read from
    pub quote_mint: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ProcessOrders<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub base_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub quote_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ListPlacedOrders<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
}

#[account]
pub struct Market {
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub orders: Vec<Order>,
    pub next_order_id: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Copy, Debug)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Order {
    pub id: u64,
    pub owner: Pubkey,
    pub side: OrderSide,
    pub amount: u64,
    pub price: u64,
}



