use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{greetings, state::PriceFeed, HelloSolanaError};

#[derive(Accounts)]
pub struct PriceUpdater<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space=size_of::<PriceFeed>() + 8
    )]
    pub price_updater: Account<'info, PriceFeed>,

    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>,
}

pub fn handle_read_price(ctx: Context<PriceUpdater>) -> Result<()> {
    greetings!(ctx.program_id);

    let price_feed = &ctx.accounts.price_updater;
    let clock = &ctx.accounts.clock;
    // Get the current timestamp
    let timestamp: i64 = clock.unix_timestamp;
    // Load the price from the price feed. Here, the price can be no older than 500 seconds.
    let price: pyth_sdk::Price = price_feed
        .get_price_no_older_than(timestamp, 30)
        .ok_or(HelloSolanaError::PythError)?;

    let confidence_interval: u64 = price.conf;

    let asset_price_full: i64 = price.price;

    let asset_exponent: i32 = price.expo;

    let asset_price = asset_price_full as f64 * 10f64.powi(asset_exponent);

    msg!("Price: {}", asset_price);
    msg!("Confidence interval: {}", confidence_interval);

    Ok(())
}

