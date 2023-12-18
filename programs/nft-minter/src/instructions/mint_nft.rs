use anchor_lang::prelude::*;
use solana_program::system_instruction;

use anchor_spl::{
        associated_token::AssociatedToken,
        token::{self, Mint, Token, TokenAccount, Transfer},
    };

use crate::constant::*;


#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: read only
    pub from: AccountInfo<'info>,

    /// CHECK: read only
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_account,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint_account,
        associated_token::authority = from,
    )]
    pub from_token_account: Account<'info, TokenAccount>,

    /// CHECK: Read only authority
    #[account(
        seeds = [
            mint_account.key().as_ref(),
            AUTHORITY_SEED.as_ref(),
        ],
        bump,
    )]
    pub spender_authourity: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {

    let transfer_instruction = system_instruction::transfer(
        ctx.accounts.user.key,
        ctx.accounts.recipient.key,
        PRICE,
    );

    // transfer SOL from user
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.recipient.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ], 
    )?;

    let authority_bump = *ctx.bumps.get("spender_authourity").unwrap();
    let authority_seeds = &[
        &ctx.accounts.mint_account.key().to_bytes(),
        AUTHORITY_SEED.as_bytes(),
        &[authority_bump]
    ];
    let signer_seeds = &[&authority_seeds[..]];

    // transfer nft to user
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.spender_authourity.to_account_info(),
            },
            signer_seeds,
        ), 
        QUANTITY_PER_MINT
    )?;

    Ok(())
}