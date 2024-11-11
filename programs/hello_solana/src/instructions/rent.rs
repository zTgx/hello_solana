use anchor_lang::prelude::*;
use anchor_lang::solana_program::rent as rent_module;

use crate::greetings;

#[derive(Accounts)]
pub struct EmptyRent {}

pub fn handle_empty_rent(ctx: Context<EmptyRent>) -> Result<()> {
    greetings!(ctx.program_id);

    let cost_of_empty_acc = rent_module::ACCOUNT_STORAGE_OVERHEAD as f64 * 
                            rent_module::DEFAULT_LAMPORTS_PER_BYTE_YEAR as f64 *
                            rent_module::DEFAULT_EXEMPTION_THRESHOLD; 

    msg!("cost to create an empty account: {}", cost_of_empty_acc);
    // 890880

    let cost_for_32_bytes = cost_of_empty_acc + 
                                32 as f64 * 
                                rent_module::DEFAULT_LAMPORTS_PER_BYTE_YEAR as f64 *
                                rent_module::DEFAULT_EXEMPTION_THRESHOLD;

    msg!("cost to create a 32 byte account: {}", cost_for_32_bytes);
    // 1,113,600 lamports

    Ok(())
}