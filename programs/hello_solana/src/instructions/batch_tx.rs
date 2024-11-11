use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::greetings;

#[derive(Accounts)]
pub struct InitializeBatchTx<'info> {
    #[account(init, payer = signer, space = size_of::<BatchData>() + 8, seeds = [b"batchtx"], bump)]
    pub pda: Account<'info, BatchData>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut)]
    pub pda: Account<'info, BatchData>,
}

#[account]
pub struct BatchData {
    pub value: u32,
}

pub fn handle_initialize_batch(ctx: Context<InitializeBatchTx>) -> Result<()> {
    greetings!(ctx.program_id);
    
    Ok(())
}

pub fn handle_batch_set(ctx: Context<Set>, new_val: u32) -> Result<()> {
    ctx.accounts.pda.value = new_val;

    Ok(())
}