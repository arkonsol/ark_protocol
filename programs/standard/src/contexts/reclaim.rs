use anchor_lang::prelude::*;
use reclaim::cpi::accounts::VerifyProof;
use reclaim::cpi::verify_proof;
use reclaim::instructions::VerifyProofArgs;
use reclaim::program::Reclaim;
use reclaim::state::ClaimData as ReclaimClaimData;
use reclaim::state::ClaimInfo as ReclaimClaimInfo;
use reclaim::state::SignedClaim as ReclaimSignedClaim;
use reclaim::state::Witness;
use reclaim::state::{Epoch, EpochConfig};
use crate::errors::RouterError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VerifyArgs {
    pub claim_info: ClaimInfo,
    pub signed_claim: SignedClaim,
}

/// Represents a signed claim
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SignedClaim {
    pub claim_data: ClaimData,
    pub signatures: Vec<[u8; 65]>,
}

/// Information about the claim
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ClaimInfo {
    pub provider: String,
    pub parameters: String,
    pub context_address: Pubkey,
    pub context_message: String,
}

/// Data of the claim
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ClaimData {
    pub identifier: [u8; 32],
    pub owner: String,
    pub timestamp: u32,
    pub epoch_index: u32,
}

#[derive(Accounts)]
pub struct Verify<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub epoch_config: Account<'info, EpochConfig>,
    pub epoch: Account<'info, Epoch>,
    pub reclaim_program: Program<'info, Reclaim>,
    pub system_program: Program<'info, System>,
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 64 + 8 + 1, // Adjust space as needed
        seeds = [b"verification", signer.key().as_ref()],
        bump
    )]
    pub verification_result: Account<'info, VerificationResult>,
}

/// Accounts required for listing verifications
#[derive(Accounts)]
pub struct ListVerifications<'info> {
    pub signer: Signer<'info>,
}

/// Accounts required for revoking a verification
#[derive(Accounts)]
pub struct RevokeVerification<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"verification", signer.key().as_ref()],
        bump,
        constraint = verification_result.signer == signer.key() @ RouterError::Unauthorized
    )]
    pub verification_result: Account<'info, VerificationResult>,
}

/// Struct to store verification results
#[account]
pub struct VerificationResult {
    pub signer: Pubkey,
    pub provider: String,
    pub verified_at: i64,
    pub is_active: bool,
}


/// Verify a claim using the Reclaim protocol
pub fn verify(ctx: Context<Verify>, args: VerifyArgs) -> Result<()> {
    let VerifyArgs {
        claim_info,
        signed_claim,
    } = args;

    // Signer and rent payer
    let signer_account_info = ctx.accounts.signer.to_account_info();
    // Program Account infos
    let reclaim_program_info = ctx.accounts.reclaim_program.to_account_info();
    
    // Proof verification
    let epoch_config_account_info = ctx.accounts.epoch_config.to_account_info();
    let epoch_account_info = ctx.accounts.epoch.to_account_info();

    // Perform the verification using Reclaim's CPI
    verify_proof(
        CpiContext::new(
            reclaim_program_info,
            VerifyProof {
                epoch_config: epoch_config_account_info,
                epoch: epoch_account_info,
                signer: signer_account_info,
            },
        ),
        VerifyProofArgs {
            claim_info: ReclaimClaimInfo {
                parameters: claim_info.parameters,
                context_message: claim_info.context_message,
                provider: claim_info.provider.clone(),
                context_address: claim_info.context_address,
            },
            signed_claim: ReclaimSignedClaim {
                claim_data: ReclaimClaimData {
                    epoch_index: signed_claim.claim_data.epoch_index,
                    timestamp: signed_claim.claim_data.timestamp,
                    identifier: signed_claim.claim_data.identifier,
                    owner: signed_claim.claim_data.owner,
                },
                signatures: signed_claim.signatures,
            },
        },
    )?;

    // Store the verification result
    let verification_result = &mut ctx.accounts.verification_result;
    verification_result.signer = ctx.accounts.signer.key();
    verification_result.provider = claim_info.provider;
    verification_result.verified_at = Clock::get()?.unix_timestamp;
    verification_result.is_active = true;

    // Emit the verification event
    emit!(VerificationCompleteEvent {
        signer: ctx.accounts.signer.key(),
        provider: verification_result.provider.clone(),
        success: true,
    });

    Ok(())
}

/// List all verifications for a specific signer
pub fn list_verifications(ctx: Context<ListVerifications>) -> Result<()> {
    // The actual listing is done client-side by fetching VerificationResult accounts
    // So this particular instruction is a no-op. It is just used as a namespace for fetching accounts
    Ok(())
}

/// Revoke a specific verification
pub fn revoke_verification(ctx: Context<RevokeVerification>) -> Result<()> {
    let verification_result = &mut ctx.accounts.verification_result;
        
    require!(
        verification_result.is_active,
        RouterError::AlreadyRevoked
    );

    verification_result.is_active = false;

    emit!(VerificationRevokedEvent {
        signer: verification_result.signer,
        provider: verification_result.provider.clone(),
    });

    Ok(())
}

pub const MAX_EPOCHS: usize = 10; // Adjust as needed
pub const MAX_WITNESSES: usize = 10; 
pub const MAX_WITNESS_ADDRESS_SIZE: usize = 32;
pub const MAX_WITNESS_URL_SIZE: usize = 128;

#[derive(Accounts)]
pub struct CreateEpochConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space =  8 + 1 + 32 + 32 + 8 + 4 + 4 + (32 * MAX_EPOCHS),
        seeds = [b"epoch_config", authority.key().as_ref()],
        bump
    )]
    pub epoch_config: Account<'info, EpochConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(epoch_index: u32)]
pub struct CreateEpoch<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"epoch_config", authority.key().as_ref()], bump)]
    pub epoch_config: Account<'info, EpochConfig>,
    #[account(
        init,
        payer = authority,
        space = 8 + // Discriminator
               1 + // bump
               32 + // epoch_config
               4 + // index
               8 + // created_at
               8 + // expired_at
               1 + // minimum_witnesses_for_claim
               4 + (MAX_WITNESSES * (4 + MAX_WITNESS_ADDRESS_SIZE + 4 + MAX_WITNESS_URL_SIZE)),
        seeds = [b"epoch", epoch_config.key().as_ref(), &epoch_index.to_le_bytes()],
        bump
    )]
    pub epoch: Account<'info, Epoch>,
    pub system_program: Program<'info, System>,
}

pub fn create_epoch_config(ctx: Context<CreateEpochConfig>) -> Result<()> {
    let epoch_config = &mut ctx.accounts.epoch_config;
    epoch_config.bump = ctx.bumps.epoch_config;
    epoch_config.create_key = ctx.accounts.authority.key();
    epoch_config.deployer = ctx.accounts.authority.key();
    epoch_config.epoch_duration_seconds = 86400; // 1 day, adjust as needed
    epoch_config.epoch_index = 0;
    epoch_config.epochs = vec![];
    Ok(())
}

pub fn create_epoch(ctx: Context<CreateEpoch>, epoch_index: u32) -> Result<()> {
    let epoch_config = &mut ctx.accounts.epoch_config;
    let epoch = &mut ctx.accounts.epoch;

    require!(epoch_index == epoch_config.epoch_index, RouterError::InvalidEpochIndex);
    require!(epoch_config.epochs.len() < MAX_EPOCHS, RouterError::MaxEpochsReached);

    epoch.bump = ctx.bumps.epoch;
    epoch.epoch_config = epoch_config.key();
    epoch.index = epoch_index;
    epoch.created_at = Clock::get()?.unix_timestamp;
    epoch.expired_at = epoch.created_at + epoch_config.epoch_duration_seconds as i64;
    epoch.minimum_witnesses_for_claim = 1; // Adjust as needed
    epoch.witnesses = vec![]; // Initialize with empty vector

    epoch_config.epoch_index += 1;
    epoch_config.epochs.push(epoch.key());

    Ok(())
}




/// The event emitted when a verification is completed
#[event]
pub struct VerificationCompleteEvent {
    pub signer: Pubkey,
    pub provider: String,
    pub success: bool,
}

/// The event emitted when a verification is revoked
#[event]
pub struct VerificationRevokedEvent {
    pub signer: Pubkey,
    pub provider: String,
}