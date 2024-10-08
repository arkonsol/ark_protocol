use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint as Mint2022, TokenAccount as TokenAccount2022};
use anchor_spl::token_2022::Token2022;
use crate::{check_proposal_status, states::{Governance, Proposal, ProposalType, ProposalStatus}};
use crate::errors::ErrorCode;
use crate::ID;
use crate::contexts::{mint_spl::{InitializeTokenBumps, MintTokensBumps}, mint_nft::MintNftBumps, mint_sbt::MintConvictionSbtBumps};
use anchor_spl::metadata::Metadata as Metaplex;
use crate::contexts::{mint_sbt::{mint_sbt, MintConvictionSbt}, mint_nft::{mint_nft, MintNft}, mint_spl::{initialize_token, InitializeToken, mint_tokens, MintTokens}};

#[derive(Accounts)]
pub struct EndAndExecuteProposal<'info> {
    #[account(mut)]
    pub proposal: Box<Account<'info, Proposal>>,
    #[account(mut)]
    pub governance: Box<Account<'info, Governance>>,
    #[account(mut)]
    pub executor: Signer<'info>,
    #[account(mut)]
    /// CHECK: We are passing in this account ourselves
    pub treasury: UncheckedAccount<'info>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    #[account(mut)]
    pub spl_mint: Account<'info, Mint>,
    /// CHECK: This account is checked in the NFT program
    pub nft_program: AccountInfo<'info>,
    #[account(mut)]
    pub nft_mint: InterfaceAccount<'info, Mint2022>,
    #[account(mut)]
    pub citizen_nft_ata: InterfaceAccount<'info, TokenAccount2022>,
    /// CHECK: This account is checked in the SBT program
    pub sbt_program: AccountInfo<'info>,
    #[account(mut)]
    pub sbt_mint: InterfaceAccount<'info, Mint2022>,
    #[account(mut)]
    pub governance_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub citizen_sbt_ata: InterfaceAccount<'info, TokenAccount2022>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_2022program: Program<'info, Token2022>,
    pub token_metadata_program: Program<'info, Metaplex>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn end_and_execute_proposal(ctx: Context<EndAndExecuteProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    {
        let governance = &mut ctx.accounts.governance;
        // Final check of proposal status
        check_proposal_status(proposal, governance)?;
    
        }
    let governance = &ctx.accounts.governance;
    let clock = Clock::get()?;

    require!(
        clock.unix_timestamp >= proposal.execution_time,
        ErrorCode::VotingPeriodNotEnded
    );

    if proposal.status == ProposalStatus::Passed {
        match proposal.proposal_type {
            ProposalType::InitializeToken => {
                let params = proposal.init_token_params.as_ref().ok_or(ErrorCode::InvalidProposalData)?;
                    // Derive the PDA for the mint
                    let (_, init_token_bump) = Pubkey::find_program_address(
                &[
                        Governance::SPL_PREFIX_SEED,
                        governance.key().as_ref(),
                        params.symbol.as_bytes(),
                    ],
                    ctx.program_id
                );

                initialize_token(
                    Context::new(
                        &ID,
                        &mut InitializeToken {
                            governance: ctx.accounts.governance.clone(),
                            payer: ctx.accounts.executor.clone(),
                            mint: ctx.accounts.spl_mint.clone(),
                            metadata: ctx.accounts.metadata.clone(),
                            token_program: ctx.accounts.token_program.clone(),
                            token_metadata_program: ctx.accounts.token_metadata_program.clone(),
                            system_program: ctx.accounts.system_program.clone(),
                            rent: ctx.accounts.rent.clone(),
                        },
                        &[],
                        InitializeTokenBumps { mint: init_token_bump },
                    ),
                    params.clone(),
                )?;
            },
            ProposalType::MintTokens => {
                let amount_to_treasury = proposal.mint_amount_treasury.ok_or(ErrorCode::InvalidProposalData)?;
                let amount_to_citizen = proposal.mint_amount_citizen.ok_or(ErrorCode::InvalidProposalData)?;
                
                mint_tokens(
                    Context::new(
                        &ID,
                        &mut MintTokens {
                            payer: ctx.accounts.executor.clone(),
                            governance: ctx.accounts.governance.clone(),
                            mint: ctx.accounts.spl_mint.clone(),
                            citizen_ata: ctx.accounts.recipient.clone(),
                            token_program: ctx.accounts.token_program.clone(),
                            associated_token_program: ctx.accounts.associated_token_program.clone(),
                            system_program: ctx.accounts.system_program.clone(),
                            rent: ctx.accounts.rent.clone(),
                            governance_ata: ctx.accounts.governance_ata.clone(),
                        },
                        &[],
                        MintTokensBumps {},
                    ),
                    amount_to_treasury,
                    amount_to_citizen,
                )?;

                let governance = &mut ctx.accounts.governance;
                
                // Update governance state
                governance.total_spl_token_supply = governance.total_spl_token_supply
                    .checked_add(amount_to_treasury)
                    .and_then(|sum| sum.checked_add(amount_to_citizen))
                    .ok_or(ErrorCode::Overflow)?;
            },
            ProposalType::MintNft => {
                let nft_args = proposal.nft_args.as_ref().ok_or(ErrorCode::InvalidProposalData)?;
                // Derive the PDA for the mint
                    let (_, mint_nft_bump) = Pubkey::find_program_address(
                &[
                        Governance::NFT_PREFIX_SEED,
                        governance.key().as_ref(),
                        nft_args.symbol.as_bytes(),
                    ],
                    ctx.program_id
                );
                mint_nft(
                    Context::new(
                        &ID,
                        &mut MintNft {
                            signer: ctx.accounts.executor.clone(),
                            governance: ctx.accounts.governance.clone(),
                            mint: ctx.accounts.nft_mint.clone(),
                            citizen_ata: ctx.accounts.citizen_nft_ata.clone(),
                            rent: ctx.accounts.rent.clone(),
                            token_program: ctx.accounts.token_2022program.clone(),
                            associated_token_program: ctx.accounts.associated_token_program.clone(),
                            system_program: ctx.accounts.system_program.clone(),
                        },
                        &[],
                        MintNftBumps { mint: mint_nft_bump },
                    ),
                    nft_args.clone(),
                )?;
            },
            ProposalType::MintSbt => {
                let sbt_args = proposal.sbt_args.as_ref().ok_or(ErrorCode::InvalidProposalData)?;
                mint_sbt(
                    Context::new(
                        &ID,
                        &mut MintConvictionSbt {
                            payer: ctx.accounts.executor.clone(),
                            governance: ctx.accounts.governance.clone(),
                            mint: ctx.accounts.sbt_mint.clone(),
                            citizen_ata: ctx.accounts.citizen_sbt_ata.clone(),
                            system_program: ctx.accounts.system_program.clone(),
                            rent: ctx.accounts.rent.clone(),
                            associated_token_program: ctx.accounts.associated_token_program.clone(),
                            token_program: ctx.accounts.token_2022program.clone(),
                        },
                        &[],
                        MintConvictionSbtBumps {},
                    ),
                    sbt_args.clone(),
                )?;
            },
            ProposalType::Transfer => {
                // Transfer funds
                let transfer_amount = proposal.transfer_amount.ok_or(ErrorCode::InvalidProposalData)?;
                transfer(
                    CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        Transfer {
                            from: ctx.accounts.treasury.to_account_info(),
                            to: ctx.accounts.recipient.to_account_info(),
                            authority: ctx.accounts.executor.to_account_info(),
                        },
                    ),
                    transfer_amount,
                )?;
            },
            ProposalType::UpdateParameter => {
                let param_name = proposal.param_name.as_ref().ok_or(ErrorCode::InvalidProposalData)?;
                let param_value = proposal.param_value.ok_or(ErrorCode::InvalidProposalData)?;
                let governance = &mut ctx.accounts.governance;
                
                match param_name.as_str() {
                    "min_stake_amount" => governance.min_stake_amount = param_value,
                    "approval_threshold" => governance.approval_threshold = param_value,
                    "collection_price" => governance.collection_price = param_value,
                    // "voting_period" => governance.voting_period = param_value as i64,
                    _ => return Err(ErrorCode::InvalidParameterName.into()),
                }
            },
        }
        proposal.status = ProposalStatus::Executed;
    } else {
        proposal.status = ProposalStatus::Rejected;
    }

    Ok(())
}