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

    pub fn emit_events(ctx: Context<Events>) -> Result<()> {
        handle_emit_events(ctx)
    }

    pub fn only_owner(ctx: Context<OnlyOwner>) -> Result<()> {
        handle_only_owner(ctx)
    }

    pub fn mapping(ctx: Context<Mappings>, key: u64) -> Result<()> {
        handle_mapping(ctx, key)
    }

    pub fn empty_rent(ctx: Context<EmptyRent>) -> Result<()> {
        handle_empty_rent(ctx)
    }

    pub fn init_account(ctx: Context<InitializeAccount>) -> Result<()> {
        handle_init_account(ctx)    
    }
    
    pub fn increase_account_size(ctx: Context<IncreaseAccountSize>) -> Result<()> {
        handle_increase_account_size(ctx)
    }

    pub fn read_balance(ctx: Context<ReadBalance>) -> Result<()> {
        handle_read_balance(ctx)
    }

    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> {
        handle_send_sol(ctx, amount)
    }

    pub fn split_sol<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SplitSol<'info>>,
        amount: u64,
    ) -> Result<()> {
        handle_split_sol(ctx, amount)
    }

    pub fn initialize_keypair(ctx: Context<InitializeKeypair>) -> Result<()> {
        handle_initialize_keypair(ctx)
    }

    pub fn initialize_pda(ctx: Context<InitializePda>) -> Result<()> {
        handle_initialize_pda(ctx)
    }
    pub fn initialize_cd_pda(ctx: Context<InitializeCdPda>) -> Result<()> {
        handle_initialize_cd_pda(ctx)
    }
    
    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        handle_donate(ctx, amount)
    }
    
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        handle_withdraw(ctx, amount)
    }
    
    pub fn initialize_batch(ctx: Context<InitializeBatchTx>) -> Result<()> {
        handle_initialize_batch(ctx)
    }
    
    pub fn batch_set(ctx: Context<Set>, new_val: u32) -> Result<()> {
        handle_batch_set(ctx, new_val)
    }

    pub fn initialize_close(ctx: Context<InitializeClose>) -> Result<()> {
        handle_initialize_close(ctx)
    }
    
    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        handle_delete(ctx)
    }


} // End hello_solana



