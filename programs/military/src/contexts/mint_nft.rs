use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use crate::states::junta::{Junta, MintNftArgs};

use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::rent::{
    DEFAULT_EXEMPTION_THRESHOLD, DEFAULT_LAMPORTS_PER_BYTE_YEAR,
};
use anchor_lang::solana_program::system_instruction;
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
use crate::states::citizen::Citizen;


#[derive(Accounts)]
#[instruction(args: MintNftArgs)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub junta: Account<'info, Junta>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer,
        extensions::metadata_pointer::authority = junta,
        extensions::metadata_pointer::metadata_address = mint,
        seeds = [Junta::NFT_PREFIX_SEED, junta.key().as_ref(), args.symbol.as_bytes()], 
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub citizen: Account<'info, Citizen>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = citizen,
        associated_token::token_program = token_program,
    )]
    pub citizen_ata: InterfaceAccount<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_nft(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let junta_key = junta.key();
    junta.symbol = args.symbol.clone();

    let seeds = &[
        Junta::NFT_PREFIX_SEED,
        junta_key.as_ref(),
        args.symbol.as_bytes(),
        &[ctx.bumps.mint]
    ];

    let signer = &[&seeds[..]];

    if junta.leader != ctx.accounts.signer.key() {
        return Err(ErrorCode::Unauthorized.into());
    }

    if junta.nft_minted >= junta.total_nft_token_supply {
        return Err(ErrorCode::SupplyReached.into());
    }

    let from_account = &ctx.accounts.citizen_ata;
    let mut to_account = junta.clone(); 

    let transfer_instruction =
        system_instruction::transfer(&from_account.key(), &to_account.key(), junta.collection_price);

    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            ctx.accounts.citizen_ata.to_account_info(),
            junta.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    to_account.resources += 1;

    let token_metadata = TokenMetadata {
        name: args.name.clone(),
        symbol: junta.symbol.clone(),
        uri: args.uri.clone(),
        ..Default::default()
    };

    let data_len = 4 + token_metadata.get_packed_len()?;

    let lamports =
        data_len as u64 * DEFAULT_LAMPORTS_PER_BYTE_YEAR * DEFAULT_EXEMPTION_THRESHOLD as u64;

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
        &junta.to_account_info().key,
        &ctx.accounts.mint.key(),
        &ctx.accounts.signer.to_account_info().key,
        args.name.clone(),
        junta.symbol.clone(),
        args.uri.clone(),
    );

    invoke_signed(
        init_token_meta_data_ix,
        &[
            ctx.accounts.mint.to_account_info().clone(),
            junta.to_account_info().clone(),
            ctx.accounts.signer.to_account_info().clone(),
        ],
        signer,
    )?;

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.citizen_ata.to_account_info(),
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
        None,
    )?;

    junta.nft_minted += 1;

    emit!(NftMinted {
        junta: junta.key(),
        mint: ctx.accounts.mint.key(),
        owner: ctx.accounts.citizen.key(),
        name: args.name,
        uri: args.uri,
        cost: junta.collection_price
    });

    Ok(())

}

#[event]
pub struct NftMinted {
    pub junta: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub name: String,
    pub uri: String,
    pub cost: u64
}


#[derive(Accounts)]
pub struct UpdateNftPrice<'info> {
    #[account(mut)]
    pub leader: Signer<'info>,
    #[account(mut, has_one = leader @ ErrorCode::Unauthorized)]
    pub junta: Account<'info, Junta>,
}

pub fn update_nft_price(ctx: Context<UpdateNftPrice>, new_price: u64) -> Result<()> {
    let junta = &mut ctx.accounts.junta;

    if junta.leader != ctx.accounts.leader.key() {
        return Err(ErrorCode::Unauthorized.into());
    }
    junta.collection_price = new_price;
    
    emit!(NftPriceUpdated {
        junta: junta.key(),
        new_price,
    });
    
    Ok(())
}

#[event]
pub struct NftPriceUpdated {
    pub junta: Pubkey,
    pub new_price: u64,
}