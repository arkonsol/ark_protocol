use anchor_lang::prelude::*;

#[account]
pub struct EconomicPolicy {
    pub policy_type: EconomicPolicyType,
    pub implemented_at: i64,
    pub income_tax_rate: u8,
    pub property_tax_rate: u8,
    pub trade_tax_rate: u8,
    pub luxury_tax_rate: u8,
    pub bump: u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum EconomicPolicyType {
    Mercantilism,
    FreeTrade,
    Protectionism,
}

impl EconomicPolicy {
    pub const SPACE: usize = 8 +  // discriminator
        1 +  // policy_type (enum)
        8 +  // implemented_at (i64)
        1 +  // income_tax_rate (u8)
        1 +  // property_tax_rate (u8)
        1 +  // trade_tax_rate (u8)
        1 +  // bump
        1;    // luxury_tax_rate (u8)
}