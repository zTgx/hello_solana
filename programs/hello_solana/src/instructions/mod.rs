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

pub mod sysvars;
pub use sysvars::*;

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

    Always,
}