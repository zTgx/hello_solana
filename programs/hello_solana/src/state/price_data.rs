use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[account]
pub struct Data {
    pub price: PriceUpdateV2,
}
