use anchor_lang::prelude::*;
use crate::greetings;

#[derive(Accounts)]
pub struct Events {}

pub fn handle_emit_events(ctx: Context<Events>) -> Result<()> {
    greetings!(ctx.program_id);

    emit!(MyEvent { value: 42 });

    Ok(())
}

#[event]
pub struct MyEvent {
    pub value: u64,
}