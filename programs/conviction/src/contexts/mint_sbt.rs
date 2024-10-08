use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::governance::{Governance, InitializeSbtArgs};
use the_ark_program::{initialize_token_metadata_extension, mint_to_token_account, set_account_or_mint_authority, update_account_lamports_to_minimum_balance, initialize_token_group_extension, initialize_non_transferrable_extension};

#[derive(Accounts)]
pub struct MintConvictionSbt<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(mut)]
    pub governance: Box<Account<'info, Governance>>,
    
    #[account(
        init,
        payer = payer,
        mint::authority = payer,
        mint::token_program = token_program,
        mint::decimals = 0,
        extensions::metadata_pointer::metadata_address = mint,
        extensions::group_pointer::group_address = mint,
        extensions::transfer_hook::authority = payer,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub citizen_ata: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

pub fn mint_sbt(ctx: Context<MintConvictionSbt>, args: InitializeSbtArgs) -> Result<()> {
    let governance = &mut ctx.accounts.governance;
    let mint = &ctx.accounts.mint;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let citizen_ata = &ctx.accounts.citizen_ata;


    initialize_token_group_extension(
        u32::MAX,
        mint,
        mint,
        payer,
        Some(citizen_ata.key()),
        token_program,
        None,
    )?;

    initialize_token_metadata_extension(
        mint,
        mint,
        payer,
        payer,
        &ctx.accounts.token_program.to_account_info(),
        args.name.clone(),
        args.symbol.clone(),
        args.uri.clone(),
    )?;

    update_account_lamports_to_minimum_balance(
        mint.to_account_info(),
        payer.to_account_info(),
        system_program.to_account_info(),
    )?;

    // let citizen_token_account = anchor_spl::associated_token::get_associated_token_address(
    //     &citizen_pubkey, 
    //     &junta_mint.key(),
    // ); // add citizen_pubkey as parameter

    mint_to_token_account(1, mint, payer, citizen_ata, token_program)?;

    set_account_or_mint_authority(
        mint,
        &payer.to_account_info(),
        None,
        AuthorityType::MintTokens,
        token_program,
    )?;

    initialize_non_transferrable_extension(mint, token_program)?;

    governance.sbt_minted += 1;
    governance.total_sbt_token_supply += 1;


    Ok(())
}
