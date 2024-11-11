use anchor_lang::error_code;

pub mod create;
pub use create::*;

pub mod initialize;
pub use initialize::*;

// pub mod price_updater;
// pub use price_updater::*;

pub mod counter;
pub use counter::*;

pub mod error;
pub use error::*;

pub mod favorites;
pub use favorites::*;

pub mod check_account;
pub use check_account::*;

pub mod create_or_close_user;
pub use create_or_close_user::*;

#[allow(deprecated)]
pub mod sysvars;
pub use sysvars::*;

pub mod events;
pub use events::*;

pub mod only_owner;
pub use only_owner::*;

pub mod mappings;
pub use mappings::*;

pub mod rent;
pub use rent::*;

pub mod realloc;
pub use realloc::*;

pub mod uncheck;
pub use uncheck::*;

pub mod send_sol;
pub use send_sol::*;

pub mod ownership;
pub use ownership::*;

pub mod crowd_fund;
pub use crowd_fund::*;

#[macro_export]
macro_rules! greetings {
    ($program_id:expr) => {
        msg!("Greetings from: {:?}", $program_id);
    };
}

#[error_code]
pub enum HelloSolanaError {

    Overflow,
    PythError,
    TryToSerializePriceAccount,

    #[msg("transfer failed")]
    TransferFailed,

    #[msg("Always errors")]
    Always,
}