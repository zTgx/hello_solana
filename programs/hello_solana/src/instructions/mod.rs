use anchor_lang::error_code;

pub mod create;
pub use create::*;

pub mod initialize;
pub use initialize::*;

pub mod price_updater;
pub use price_updater::*;

pub mod counter;
pub use counter::*;

pub mod error;
pub use error::*;

#[macro_export]
macro_rules! greetings {
    ($program_id:expr) => {
        msg!("Greetings from: {:?}", $program_id);
    };
}

#[error_code]
pub enum HelloSolanaError {
    Overflow,
    
    Always,
}