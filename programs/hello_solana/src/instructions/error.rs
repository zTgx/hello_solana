use anchor_lang::prelude::*;
use crate::{greetings, HelloSolanaError};

#[derive(Accounts)]
pub struct ErrorExample {}

pub fn handle_error_example(ctx: Context<ErrorExample>) -> Result<()> {
    greetings!(ctx.program_id);

    Err(HelloSolanaError::Always.into())
}