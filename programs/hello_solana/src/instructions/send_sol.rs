use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::greetings;
use crate::HelloSolanaError;

#[derive(Accounts)]
pub struct SendSol<'info> {
    /// CHECK: we do not read or write the data of this account
    #[account(mut)]
    recipient: UncheckedAccount<'info>,
    
    // system program transfers SOL from one account to another. 
    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

pub fn handle_send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> {
    greetings!(ctx.program_id);

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(), 

        system_program::Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
        }
    );

    let res = system_program::transfer(cpi_context, amount);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(HelloSolanaError::TransferFailed);
    }
}

#[derive(Accounts)]
pub struct SplitSol<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handle_split_sol<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, SplitSol<'info>>,
    amount: u64,
) -> Result<()> {

    let amount_each_gets = amount / ctx.remaining_accounts.len() as u64;
    let system_program = &ctx.accounts.system_program;

    // note the keyword `remaining_accounts`
    for recipient in ctx.remaining_accounts {
        let cpi_accounts = system_program::Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: recipient.to_account_info(),
        };
        let cpi_program = system_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        let res = system_program::transfer(cpi_context, amount_each_gets);
        if !res.is_ok() {
            return err!(HelloSolanaError::TransferFailed);
        }
    }

    Ok(())
}