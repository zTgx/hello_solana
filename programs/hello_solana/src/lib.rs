#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
use instructions::*;

declare_id!("6aSQULbw3cswNouuCCVFD2M9ZbeZmXoUkjNqVceGofRS");

#[program]
pub mod hello_solana {
    use super::*;

    // initialized handler
    pub fn initialized(ctx: Context<Initialized>) -> Result<()> {
        msg!("Greetings from: {:?}", &id());

        initialized::initialized(ctx)
    }
    
    pub fn update_price(ctx: Context<PriceUpdater>, feed_id: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        price_updater::handler(ctx, &feed_id)
    }
}



