use anchor_lang::prelude::*;

#[account]
pub struct EscrowInfo {
pub total_trades: u64,
pub trades: Vec<Pubkey>,
pub services: Vec<Pubkey>,
pub total_services: u64,
pub total_fees_collected: u64,
pub total_amount_transferred: u64,
}

impl EscrowInfo {
    pub const LEN: usize = 8    // total_trades
        + 8                    // total_services
        + 8                    // total_fees_collected
        + 8                    // total_amount_transferred
        + 4                    // length prefix for trades Vec<Pubkey>
        + 4                    // length prefix for services Vec<Pubkey>
        + 32 * MAX_TRADES      // maximum number of trades
        + 32 * MAX_SERVICES    // maximum number of services
        + 24                   // overhead for trades Vec<Pubkey> (3 * 8 bytes for pointer, length, capacity)
        + 24;                  // overhead for services Vec<Pubkey> (3 * 8 bytes for pointer, length, capacity)
}

const MAX_TRADES: usize = 10;  
const MAX_SERVICES: usize = 10; 
