use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{greetings, state::Mapping};

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Mappings<'info> {

    #[account(init,
              payer = signer,
              space = size_of::<Mapping>() + 8,
              seeds=[&key.to_le_bytes().as_ref()],
              bump)]
    val: Account<'info, Mapping>,
    
    #[account(mut)]
    signer: Signer<'info>,
    
    system_program: Program<'info, System>,
}

pub fn handle_mapping(ctx: Context<Mappings>, key: u64) -> Result<()> {
    greetings!(ctx.program_id);
    msg!("Key: {}", key);

    Ok(())
}

