
use anchor_lang::prelude::*;

use crate::greetings;

#[derive(Accounts)]
pub struct InitializeKeypair<'info> {
    #[account(init, payer = signer, space = 8)]
    keypair: Account<'info, Keypair>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePda<'info> {
    #[account(init, payer = signer, space = 8, seeds = [], bump)]
    pda: Account<'info, Pda>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Keypair();

#[account]
pub struct Pda();

pub fn handle_initialize_keypair(ctx: Context<InitializeKeypair>) -> Result<()> {
    greetings!(ctx.program_id);

    Ok(())
}

pub fn handle_initialize_pda(ctx: Context<InitializePda>) -> Result<()> {
    greetings!(ctx.program_id);

    Ok(())
}