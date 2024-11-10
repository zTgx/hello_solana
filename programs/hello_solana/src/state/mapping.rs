use anchor_lang::prelude::*;

#[account]
pub struct Mapping {
    value: u64,
}