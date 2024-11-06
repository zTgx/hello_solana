use anchor_lang::prelude::*;
use crate::{greetings, state::counter::Counter, HelloSolanaError};

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 8 + Counter::INIT_SPACE,
        payer = payer
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

pub fn handle_initialize_counter(ctx: Context<InitializeCounter>, max: u64) -> Result<()> {
    greetings!(ctx.program_id);

    let counter = &mut ctx.accounts.counter;
    counter.max_count = max;

    Ok(())
}

pub fn handle_increment(ctx: Context<Increment>) -> Result<()> {
    greetings!(ctx.program_id);
    
    
    if ctx.accounts.counter.count >= ctx.accounts.counter.max_count {
        return Err(HelloSolanaError::Overflow.into());
    }
    ctx.accounts.counter.count += 1;
    
    Ok(())
}