use std::{mem::size_of, str::FromStr};
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct InitializeCdPda<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space=size_of::<CdPda>() + 8, seeds=[b"crowdfund"], bump)]
    pub pda: Account<'info, CdPda>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pda: Account<'info, CdPda>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, address = Pubkey::from_str("8QB5VckaW3CWv4oZWiMLs1GkdrR5pVcjarAS1U6rG6Wh").unwrap())]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pda: Account<'info, CdPda>,
}

#[account]
pub struct CdPda {}

pub fn handle_initialize_cd_pda(ctx: Context<InitializeCdPda>) -> Result<()> {
    let _initialized_pda = &mut ctx.accounts.pda;
    Ok(())
}

pub fn handle_donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        
        system_program::Transfer {
            from: ctx.accounts.signer.to_account_info().clone(),
            to: ctx.accounts.pda.to_account_info().clone(),
        },
    );

    system_program::transfer(cpi_context, amount)?;

    Ok(())
}

pub fn handle_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    ctx.accounts.pda.sub_lamports(amount)?;
    ctx.accounts.signer.add_lamports(amount)?;

    // in anchor 0.28 or lower, use the following syntax:
    // **ctx.accounts.pda.to_account_info().try_borrow_mut_lamports()? -= amount;
    // **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? += amount;
    Ok(())
}


