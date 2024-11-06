use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::get_feed_id_from_hex;
use std::mem::size_of;
use crate::{greetings, state::Data};

#[derive(Accounts)]
pub struct PriceUpdater<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space=size_of::<Data>() + 8
    )]
    pub price_updater: Account<'info, Data>,

    pub system_program: Program<'info, System>,
}

pub fn handle_price_update(ctx: Context<PriceUpdater>, feed_id: &str) -> Result<()> {
    greetings!(ctx.program_id);

    let price_updater = &mut ctx.accounts.price_updater;
    let max_age: u64 = 30;
    let feed_id: [u8; 32] = get_feed_id_from_hex(feed_id)?;
    let price = price_updater.price.get_price_no_older_than(&Clock::get()?, max_age, &feed_id)?;

    msg!("The price is ({} / {}) * 10^{}", price.price, price.conf, price.exponent);

    Ok(())
}

