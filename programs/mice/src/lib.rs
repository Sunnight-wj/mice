use anchor_lang::prelude::*;

mod instructions;
mod constant;


declare_id!("B9xWniPBaNDGBDi1cxvTwFogiJtSEbZuuoqV2esqnyXp");

#[program]
pub mod mice {
    pub use super::instructions::*;
    use super::*;

    pub fn approve_nft(ctx: Context<ApproveNFT>, amount: u64) -> Result<()> {
        instructions::approve_nft(ctx, amount)
    }

    pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {
        instructions::mint_nft(ctx)
    }
}


