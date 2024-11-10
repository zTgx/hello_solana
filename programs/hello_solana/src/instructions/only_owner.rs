use anchor_lang::prelude::*;

// NOTE: Replace with your wallet's public key
const OWNER: &str = "8QB5VckaW3CWv4oZWiMLs1GkdrR5pVcjarAS1U6rG6Wh";

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

// An enum for custom error codes
#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

#[access_control(check(&ctx))]
pub fn handle_only_owner(ctx: Context<OnlyOwner>) -> Result<()> {
    // Function logic...

    msg!("Holla, I'm the owner.");
    Ok(())
}