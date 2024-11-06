use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialized {}

pub fn initialized(ctx: Context<Initialized>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
