use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::greetings;

#[derive(Accounts)]
pub struct InitializeClose<'info> {
    #[account(init, payer = signer, space = size_of::<ThePda>() + 8, seeds = [b"close"], bump)]
    pub the_pda: Account<'info, ThePda>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut, close = signer, )]
    pub the_pda: Account<'info, ThePda>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
pub struct ThePda {
    pub x: u32,
}

pub fn handle_initialize_close(ctx: Context<InitializeClose>) -> Result<()> {
    greetings!(ctx.program_id);

    Ok(())
}

pub fn handle_delete(ctx: Context<Delete>) -> Result<()> {
    greetings!(ctx.program_id);

    Ok(())
}
