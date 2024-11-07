use anchor_lang::prelude::*;
use crate::greetings;

#[derive(Accounts)]
pub struct Initialized {}

pub fn handle_initialize(ctx: Context<Initialized>) -> Result<()> {
    greetings!(ctx.program_id);
    
    Ok(())
}
