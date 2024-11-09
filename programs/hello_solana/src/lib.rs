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
    
    // pub fn price_update(ctx: Context<PriceUpdater>) -> Result<()> {
    //     handle_read_price(ctx)
    // }

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

    pub fn error_example(ctx: Context<ErrorExample>) -> Result<()> {
        handle_error_example(ctx)
    }

    pub fn set_favorites(ctx: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        handle_set_favorites(ctx, number, color, hobbies)
    }

    pub fn check_accounts(ctx: Context<CheckingAccounts>) -> Result<()> {
        handle_check_accounts(ctx)
    }

    pub fn create_user(ctx: Context<CreateUserContext>, name: String) -> Result<()> {
        handle_create_user(ctx, name)
    }

    pub fn close_user(ctx: Context<CloseUserContext>) -> Result<()> {
        handle_close_user(ctx)
    }

    pub fn system_vars(ctx: Context<SystemVars>) -> Result<()> {
        handle_system_vars(ctx)
    }


} // End hello_solana



