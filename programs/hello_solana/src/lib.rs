#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
use instructions::*;

declare_id!("6aSQULbw3cswNouuCCVFD2M9ZbeZmXoUkjNqVceGofRS");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialized>) -> Result<()> {
        handle_initialize(ctx)
    }
    
    pub fn price_update(ctx: Context<PriceUpdater>, feed_id: String) -> Result<()> {
        handle_price_update(ctx, &feed_id)
    }

    pub fn create_address_info(
        ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        handle_create_address_info(ctx, name, house_number, street, city)
    }

    pub fn initialize_counter(ctx: Context<InitializeCounter>, max: u64) -> Result<()> {
        handle_initialize_counter(ctx, max)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        handle_increment(ctx)
    }










} // End hello_solana



