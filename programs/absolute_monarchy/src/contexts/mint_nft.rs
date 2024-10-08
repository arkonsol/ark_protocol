use anchor_lang::prelude::*;
use crate::states::{Kingdom, MintNftArgs};

use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::set_authority;
use anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_2022::{mint_to, MintTo};
use anchor_spl::token_interface::{spl_token_metadata_interface, Mint};
use anchor_spl::token_interface::{SetAuthority, TokenAccount};
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

#[derive(Accounts)]
#[instruction(args: MintNftArgs)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer,
        seeds = [Kingdom::NFT_PREFIX_SEED, kingdom.key().as_ref(), args.symbol.as_bytes()], 
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub subject_token_account: InterfaceAccount<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_nft(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;
    let kingdom_key = kingdom.key();
    kingdom.nft_symbol = args.symbol.clone();

    let seeds = &[
        Kingdom::NFT_PREFIX_SEED,
        kingdom_key.as_ref(),
        args.symbol.as_bytes(),
        &[ctx.bumps.mint]
    ];

    let signer = &[&seeds[..]];

    // Transfer collection price to the kingdom
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: kingdom.to_account_info(),
            },
        ),
        kingdom.collection_price,
    )?;

    kingdom.royal_treasury += kingdom.collection_price;

    let token_metadata = TokenMetadata {
        name: args.name.clone(),
        symbol: args.symbol.clone(),
        uri: args.uri.clone(),
        ..Default::default()
    };

    let data_len = 4 + token_metadata.get_packed_len()?;
    let lamports = Rent::get()?.minimum_balance(data_len);

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.mint.to_account_info(),
            },
        ),
        lamports,
    )?;

    let init_token_meta_data_ix = &spl_token_metadata_interface::instruction::initialize(
        &Token2022::id(),
        &ctx.accounts.mint.key(),
        &ctx.accounts.signer.key(),
        &ctx.accounts.mint.key(),
        &ctx.accounts.signer.key(),
        args.name.clone(),
        args.symbol.clone(),
        args.uri.clone(),
    );

    invoke_signed(
        init_token_meta_data_ix,
        &[
            ctx.accounts.mint.to_account_info(),
            kingdom.to_account_info(),
            ctx.accounts.signer.to_account_info(),
        ],
        signer,
    )?;

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.subject_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ),
        1,
    )?;

    set_authority(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                current_authority: ctx.accounts.signer.to_account_info(),
                account_or_mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        AuthorityType::MintTokens,
        Some(kingdom.key()),
    )?;
    
    kingdom.nft_mint = Some(ctx.accounts.mint.key());
    kingdom.nft_minted += 1;
    kingdom.total_subjects += 1;

    emit!(NftMinted {
        absolute_monarchy_kingdom: kingdom.key(),
        mint: ctx.accounts.mint.key(),
        owner: ctx.accounts.subject_token_account.key(),
        name: args.name.clone(),
        symbol: args.symbol.clone(),
        uri: args.uri.clone(),
        cost: kingdom.collection_price
    });

    Ok(())
}

#[event]
pub struct NftMinted {
    pub absolute_monarchy_kingdom: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub cost: u64
}