use anchor_lang::prelude::*;

#[account]
pub struct MyStorage {
    x: u64,
}