use anchor_lang::prelude::*;
use crate::states::governance::DemographicQuotas;

#[account]
pub struct PaginationState {
    pub governance_pool: Pubkey,
    pub current_index: u32,
    pub current_citizen_in_index: u32,
    pub selected_citizens: Vec<Pubkey>,
    pub demographic_counts: DemographicQuotas,
    pub bump: u8,
}
impl PaginationState {
    pub const MAX_ASSEMBLY_SIZE: usize = 5;

    pub const SPACE: usize = 8 + // discriminator
        32 + // governance_pool
        4 + // current_index
        4 + // current_citizen_in_index
        4 + (Self::MAX_ASSEMBLY_SIZE * 32) + // selected_citizens
        (8 * 1) + (5 * 1) + (4 * 1) + // demographic_counts
        1;
}