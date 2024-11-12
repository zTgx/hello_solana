use anchor_lang::prelude::*;

declare_id!("H5sX5Qm9UcrHXo3MS6HWUcQ8GnuUi8X5HyHDce3EKMBb");

#[program]
pub mod rpg {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
