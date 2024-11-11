use std::mem::size_of;

use anchor_lang::prelude::*;

use crate::{greetings, state::MyStorage};

#[derive(Accounts)]
pub struct IncreaseAccountSize<'info> {

    #[account(mut,
              // ***** 1,000 BYTE INCREMENT IS OVER HERE *****
              realloc = size_of::<MyStorage>() + 8 + 1000,
              realloc::payer = signer,
              realloc::zero = false,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,
    
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {

    #[account(init,
              payer = signer,
              space=size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,
    
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_init_account(ctx: Context<InitializeAccount>) -> Result<()> {
    greetings!(ctx.program_id);

    Ok(())
}

pub fn handle_increase_account_size(ctx: Context<IncreaseAccountSize>) -> Result<()> {
    greetings!(ctx.program_id);
    
    Ok(())
}