use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ReadBalance<'info> {
    /// CHECK: although we read this account's balance, we don't do anything with the information
    pub acct: UncheckedAccount<'info>,
}

// The UncheckedAccount type tells to Anchor to not check if the account being read is owned by the program.
// The #[account] struct tells Anchor how to deserialize an account holding data. 

pub fn handle_read_balance(ctx: Context<ReadBalance>) -> Result<()> {
    let balance = ctx.accounts.acct.to_account_info().lamports();

    msg!("balance in Lamports is {}", balance);
    Ok(())
}
