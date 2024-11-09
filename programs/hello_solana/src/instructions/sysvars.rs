use anchor_lang::{prelude::*, solana_program::sysvar::recent_blockhashes::RecentBlockhashes};
use crate::{greetings, id};

// Account validation in Anchor is done using the types and constraints specified in the #[derive(Accounts)] structs
// This is a simple example and does not include all possible constraints and types
#[derive(Accounts)]
pub struct SystemVars<'info> {
    /// CHECK:
    pub recent_blockhashes: AccountInfo<'info>,
}

pub fn handle_system_vars(ctx: Context<SystemVars>) -> Result<()> {
    greetings!(id());

    let clock: Clock = Clock::get()?;
    msg!(
        "Block timestamp: {}", // Time deltas however can be negative.
        // Get block.timestamp
        clock.unix_timestamp,
    );

    let arr = [ctx.accounts.recent_blockhashes.clone()];
    let accounts_iter = &mut arr.iter();
    let sh_sysvar_info = next_account_info(accounts_iter)?;
    let recent_blockhashes = RecentBlockhashes::from_account_info(sh_sysvar_info)?;
    let data = recent_blockhashes.last().unwrap();
    msg!("The recent block hash is: {:?}", data.blockhash);

    Ok(())
}