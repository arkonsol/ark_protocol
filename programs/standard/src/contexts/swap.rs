// use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
// use anchor_spl::associated_token::AssociatedToken;
// use pyth_sdk_solana::state::SolanaPriceAccount;

// // Constants
// const RATE_PRECISION: u64 = 1_000_000;  // 6 decimal places
// const FEE_PRECISION: u64 = 10_000;      // 4 decimal places
// const MAX_RATE_AGE: i64 = 60;           // 60 seconds


// pub fn initialize_pool(
//         ctx: Context<InitializePool>,
//         initial_liquidity_amount: u64,
//         swap_fee: u64,
//     ) -> Result<()> {
//         let pool = &mut ctx.accounts.pool;
//         pool.token_a_mint = ctx.accounts.token_a_mint.key();
//         pool.token_b_mint = ctx.accounts.token_b_mint.key();
//         pool.token_a_reserve = ctx.accounts.token_a_reserve.key();
//         pool.token_b_reserve = ctx.accounts.token_b_reserve.key();
//         pool.swap_fee = swap_fee;
//         pool.total_liquidity = initial_liquidity_amount;

//         // Transfer initial liquidity
//         token::transfer(
//             CpiContext::new(
//                 ctx.accounts.token_program.to_account_info(),
//                 Transfer {
//                     from: ctx.accounts.user_token_a.to_account_info(),
//                     to: ctx.accounts.token_a_reserve.to_account_info(),
//                     authority: ctx.accounts.user.to_account_info(),
//                 },
//             ),
//             initial_liquidity_amount,
//         )?;

//         token::transfer(
//             CpiContext::new(
//                 ctx.accounts.token_program.to_account_info(),
//                 Transfer {
//                     from: ctx.accounts.user_token_b.to_account_info(),
//                     to: ctx.accounts.token_b_reserve.to_account_info(),
//                     authority: ctx.accounts.user.to_account_info(),
//                 },
//             ),
//             initial_liquidity_amount,
//         )?;

//         // Trying out something different
//         // let cpi_accounts = anchor_spl::associated_token::Create {
//         //     payer: ctx.accounts.user.to_account_info(),
//         //     associated_token: ctx.accounts.user.to_account_info(),
//         //     authority: ctx.accounts.user.to_account_info(),
//         //     mint: ctx.accounts.lp_mint.to_account_info(),
//         //     system_program: ctx.accounts.system_program.to_account_info(),
//         //     token_program: ctx.accounts.token_program.to_account_info(),
//         // };
//         // let cpi_program = ctx.accounts.associated_token_program.to_account_info();
//         // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
//         // anchor_spl::associated_token::create(cpi_ctx)?;
    
//         // // Mint LP tokens to the user
//         // let user_lp_token = anchor_spl::associated_token::get_associated_token_address(
//         //     &ctx.accounts.user.key(),
//         //     &ctx.accounts.lp_mint.key(),
//         // );

//         // Mint LP tokens to the user
//         token::mint_to(
//             CpiContext::new(
//                 ctx.accounts.token_program.to_account_info(),
//                 token::MintTo {
//                     mint: ctx.accounts.lp_mint.to_account_info(),
//                     to: ctx.accounts.user_lp_token.to_account_info(),
//                     authority: pool.to_account_info(),
//                 },
//             ),
//             initial_liquidity_amount,
//         )?;

//         Ok(())
//     }


// pub fn swap(
//         ctx: Context<Swap>,
//         amount_in: u64,
//         minimum_amount_out: u64,
//     ) -> Result<()> {
//         let pool = &ctx.accounts.pool;

//         // Check if the price feed is fresh
//         let price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.pyth_price_feed).unwrap();
//         let current_timestamp = Clock::get()?.unix_timestamp;
//         let price = price_feed.get_price_no_older_than(current_timestamp, MAX_RATE_AGE.try_into().unwrap()).unwrap();

//         // Calculate the amount out based on constant product formula
//         let reserve_a = ctx.accounts.token_a_reserve.amount;
//         let reserve_b = ctx.accounts.token_b_reserve.amount;
//         let amount_out = calculate_amount_out(amount_in, reserve_a, reserve_b, pool.swap_fee)?;

//         // Check slippage
//         if amount_out < minimum_amount_out {
//             return Err(ErrorCode::ExcessiveSlippage.into());
//         }

//         // Transfer tokens from user to pool
//         token::transfer(
//             CpiContext::new(
//                 ctx.accounts.token_program.to_account_info(),
//                 Transfer {
//                     from: ctx.accounts.user_token_a.to_account_info(),
//                     to: ctx.accounts.token_a_reserve.to_account_info(),
//                     authority: ctx.accounts.user.to_account_info(),
//                 },
//             ),
//             amount_in,
//         )?;

//         // Transfer tokens from pool to user
//         token::transfer(
//             CpiContext::new_with_signer(
//                 ctx.accounts.token_program.to_account_info(),
//                 Transfer {
//                     from: ctx.accounts.token_b_reserve.to_account_info(),
//                     to: ctx.accounts.user_token_b.to_account_info(),
//                     authority: pool.to_account_info(),
//                 },
//                 &[&[b"pool", &[ctx.bumps.pool]]],
//             ),
//             amount_out,
//         )?;

//         // Emit swap event
//         emit!(SwapEvent {
//             user: ctx.accounts.user.key(),
//             amount_in,
//             amount_out,
//             fee: amount_in * pool.swap_fee / FEE_PRECISION,
//         });

//         Ok(())
// }

//     // Add more instructions for adding/removing liquidity, updating fees, etc.


// #[derive(Accounts)]
// pub struct InitializePool<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,
//     #[account(
//         init,
//         payer = user,
//         space = 8 + 32 + 32 + 32 + 32 + 8 + 8,
//         seeds = [b"pool"],
//         bump
//     )]
//     pub pool: Box<Account<'info, Pool>>,
//     /// CHECK: This account is not read or written in the program
//     pub token_a_mint: UncheckedAccount<'info>,
//     /// CHECK: This account is not read or written in the program
//     pub token_b_mint: UncheckedAccount<'info>,
//     #[account(
//         init,
//         payer = user,
//         associated_token::mint = token_a_mint,
//         associated_token::authority = pool,
//     )]
//     pub token_a_reserve: Box<Account<'info, TokenAccount>>,
//     #[account(
//         init,
//         payer = user,
//         associated_token::mint = token_b_mint,
//         associated_token::authority = pool,
//     )]
//     pub token_b_reserve: Box<Account<'info, TokenAccount>>,
//     #[account(
//         init,
//         payer = user,
//         mint::decimals = 6,
//         mint::authority = pool,
//     )]
//     pub lp_mint: Box<Account<'info, Mint>>,
//     #[account(
//         init,
//         payer = user,
//         associated_token::mint = lp_mint,
//         associated_token::authority = user,
//     )]
//     pub user_lp_token: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub user_token_a: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub user_token_b: Box<Account<'info, TokenAccount>>,
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
//     pub rent: Sysvar<'info, Rent>,
// }

// #[derive(Accounts)]
// pub struct Swap<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,
//     #[account(
//         seeds = [b"pool"],
//         bump,
//     )]
//     pub pool: Account<'info, Pool>,
//     #[account(
//         mut,
//         constraint = token_a_reserve.key() == pool.token_a_reserve
//     )]
//     pub token_a_reserve: Account<'info, TokenAccount>,
//     #[account(
//         mut,
//         constraint = token_b_reserve.key() == pool.token_b_reserve
//     )]
//     pub token_b_reserve: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub user_token_a: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub user_token_b: Account<'info, TokenAccount>,
//     /// CHECK: This account is not read or written in the program
//     pub pyth_price_feed: AccountInfo<'info>,
//     pub token_program: Program<'info, Token>,
// }

// #[account]
// pub struct Pool {
//     pub token_a_mint: Pubkey,
//     pub token_b_mint: Pubkey,
//     pub token_a_reserve: Pubkey,
//     pub token_b_reserve: Pubkey,
//     pub swap_fee: u64,
//     pub total_liquidity: u64,
// }

// #[error_code]
// pub enum ErrorCode {
//     #[msg("The provided slippage tolerance has been exceeded.")]
//     ExcessiveSlippage,
//     #[msg("Insufficient liquidity for this trade.")]
//     InsufficientLiquidity,
// }

// #[event]
// pub struct SwapEvent {
//     pub user: Pubkey,
//     pub amount_in: u64,
//     pub amount_out: u64,
//     pub fee: u64,
// }

// // Helper function to calculate the amount out based on constant product formula
// fn calculate_amount_out(amount_in: u64, reserve_a: u64, reserve_b: u64, swap_fee: u64) -> Result<u64> {
//     let amount_in_with_fee = amount_in * (FEE_PRECISION - swap_fee) / FEE_PRECISION;
//     let numerator = amount_in_with_fee * reserve_b;
//     let denominator = reserve_a + amount_in_with_fee;
    
//     if denominator == 0 {
//         return Err(ErrorCode::InsufficientLiquidity.into());
//     }
    
//     Ok(numerator / denominator)
// }