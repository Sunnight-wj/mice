use anchor_lang::prelude::*;
use solana_program::system_instruction;

use crate::constant::*;


#[derive(Accounts)]
pub struct TransferSOL<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: read only
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn transfer_sol(ctx: Context<TransferSOL>) -> Result<()> {

    let transfer_instruction = system_instruction::transfer(
        ctx.accounts.user.key,
        ctx.accounts.recipient.key,
        TRANSFER_AMOUNT,
    );

    // transfer SOL from user
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.recipient.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[], 
    )?;

    Ok(())
}