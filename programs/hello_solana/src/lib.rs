use std::mem::size_of;

use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};

declare_id!("6aSQULbw3cswNouuCCVFD2M9ZbeZmXoUkjNqVceGofRS");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn ddd(ctx: Context<DD>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        Ok(())
    }

    pub fn sample(ctx: Context<DD>) -> Result<()> {
        msg!("hell sample..");
        let price_updater = &mut ctx.accounts.price_updater;
        let max_age: u64 = 30;
        let feed_id: [u8; 32] = get_feed_id_from_hex("0x097d687437374051c75160d648800f021086bc8edf469f11284491fda8192315")?;
        let price = price_updater.price.get_price_no_older_than(&Clock::get()?, max_age, &feed_id)?;

        msg!("The price is ({} / {}) * 10^{}", price.price, price.conf, price.exponent);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct DD<'info> {
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

#[account]
pub struct Data {
    pub price: PriceUpdateV2,
}
