use anchor_lang::prelude::*;

use anchor_spl::{
        associated_token::AssociatedToken,
        token::{self, Mint, Token, TokenAccount, Approve },
    };

use crate::constant::*;

#[derive(Accounts)]
pub struct ApproveNFT<'info> {

    pub owner: Signer<'info>,

    pub mint_account: Account<'info, Mint>,

    /// CHECK: Read only authority
    #[account(
        seeds = [
            mint_account.key().as_ref(),
            AUTHORITY_SEED.as_ref(),
        ],
        bump,
    )]
    pub spender_authourity: AccountInfo<'info>,

    #[account(
        associated_token::mint = mint_account,
        associated_token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn approve_nft(ctx: Context<ApproveNFT>, amount: u64) -> Result<()> {

    token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Approve {
                to: ctx.accounts.owner_token_account.to_account_info(),
                delegate: ctx.accounts.spender_authourity.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}







