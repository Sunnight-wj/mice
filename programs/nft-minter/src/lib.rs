use anchor_lang::prelude::*;

mod instructions;
mod constant;


declare_id!("2dZCKmv4jwqCFKoT82yobCJqBcbp3FD4JZLLQXLqbNMK");

#[program]
pub mod nft_minter {
    pub use super::instructions::*;
    use super::*;

    pub fn approve_nft(ctx: Context<ApproveNFT>, amount: u64) -> Result<()> {
        instructions::approve_nft(ctx, amount)
    }

    pub fn mint_sft(ctx: Context<MintNFT>) -> Result<()> {
        instructions::mint_nft(ctx)
    }
}


