use anchor_lang::prelude::*;

pub const MAX_INSTRUCTION_DATA_SIZE: usize = 1024;

pub trait GovernmentProgram {
    fn make_decision(&self, context: InstructionContext) -> Result<Decision>;

    fn delegate_authority<'info>(
        &self,
        delegate: &AccountInfo<'info>,
        authority: &AccountInfo<'info>,
        system_program: &AccountInfo<'info>
    ) -> Result<()>;

    fn revoke_delegate<'info>(
        &self,
        authority: &AccountInfo<'info>,
        system_program: &AccountInfo<'info>
    ) -> Result<()>;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InstructionContext {
    pub program_id: Pubkey,
    pub instruction_data: Vec<u8>,
    pub signer: Pubkey,  // The account that signed the original transaction
    pub accounts: Vec<Pubkey>,  // List of accounts involved in the instruction
    pub block_time: i64,  // Timestamp of the block (useful for time-sensitive decisions)
    pub instruction_index: u8,  // Index of this instruction in the transaction
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GovernmentState {
    pub authority: Pubkey,
    pub delegated_authority: Option<Pubkey>,
    pub program_type: GovernmentTypes,
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Policy {
    pub approval_threshold: u8, 
    pub voting_period: i64,  
}

pub fn is_authorized(authority: &Pubkey, signer: &Pubkey) -> bool {
    authority == signer
}

pub fn validate_instruction_data(data: &[u8]) -> Result<()> {
    if data.len() > MAX_INSTRUCTION_DATA_SIZE {
        return Err(GovernmentError::InvalidInstructionData.into());
    }
    Ok(())
}

pub fn find_delegation_address(
    authority: &Pubkey,
    delegate: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"delegation", authority.as_ref(), delegate.as_ref()],
        program_id,
    )
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GovernmentTypes {
    AbsoluteMonarchy,
    Conviction,
    Sortition,
    Sociocracy,
    FlatDAO,
    MilitaryJunta,
    Polycentric
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Decision {
    Approve,
    Reject,
}

#[error_code]
pub enum GovernmentError {
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    #[msg("Government program is inactive")]
    InactiveGovernment,
}