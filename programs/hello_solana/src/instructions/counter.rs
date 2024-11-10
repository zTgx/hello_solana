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

// Derive Macro: Accounts
// A data structure of validated accounts that can be deserialized from the input to a Solana program.
#[derive(Accounts)]
pub struct Increment<'info> {
    // Attribute Macro: #[account]
    // An attribute for a data structure representing a Solana account.
    // #[account] generates trait implementations for the following traits:
    // AccountSerialize
    // AccountDeserialize
    // AnchorSerialize
    // AnchorDeserialize
    // Clone
    // Discriminator
    // Owner
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