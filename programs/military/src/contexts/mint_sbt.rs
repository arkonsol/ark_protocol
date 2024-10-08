use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::junta::{Junta, InitializeSbtArgs};
use the_ark_program::{initialize_token_metadata_extension, mint_to_token_account, set_account_or_mint_authority, update_account_lamports_to_minimum_balance, initialize_token_group_extension, initialize_non_transferrable_extension};
use crate::errors::ErrorCode;
use crate::states::citizen::Citizen;

#[derive(Accounts)]
pub struct MintJuntaSbt<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    
    #[account(
        init,
        payer = payer,
        mint::authority = payer,
        mint::token_program = token_program,
        mint::decimals = 0,
        extensions::metadata_pointer::metadata_address = junta_mint,
        extensions::group_pointer::group_address = junta_mint,
        extensions::transfer_hook::authority = junta,
    )]
    pub junta_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub citizen: Account<'info, Citizen>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = junta_mint,
        associated_token::authority = citizen,
        associated_token::token_program = token_program,
    )]
    pub citizen_ata: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

pub fn mint_sbt(ctx: Context<MintJuntaSbt>, args: InitializeSbtArgs) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let junta_mint = &ctx.accounts.junta_mint;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let citizen_ata = &ctx.accounts.citizen_ata;


    if junta.leader != ctx.accounts.payer.key() {
        return Err(ErrorCode::Unauthorized.into());
    }

    if junta.sbt_minted >= junta.total_sbt_token_supply {
        return Err(ErrorCode::SupplyReached.into());
    }

    initialize_token_group_extension(
        u32::MAX,
        junta_mint,
        junta_mint,
        payer,
        Some(junta.leader.key()),
        token_program,
        None,
    )?;

    initialize_token_metadata_extension(
        junta_mint,
        junta_mint,
        payer,
        payer,
        &ctx.accounts.token_program.to_account_info(),
        args.name.clone(),
        junta.symbol.clone(),
        args.uri.clone(),
    )?;

    update_account_lamports_to_minimum_balance(
        junta_mint.to_account_info(),
        payer.to_account_info(),
        system_program.to_account_info(),
    )?;

    // let citizen_token_account = anchor_spl::associated_token::get_associated_token_address(
    //     &citizen_pubkey, 
    //     &junta_mint.key(),
    // ); // add citizen_pubkey as parameter

    mint_to_token_account(1, junta_mint, payer, citizen_ata, token_program)?;

    set_account_or_mint_authority(
        junta_mint,
        &payer.to_account_info(),
        None,
        AuthorityType::MintTokens,
        token_program,
    )?;

    initialize_non_transferrable_extension(junta_mint, token_program)?;

    junta.sbt_minted += 1;

    Ok(())
}
