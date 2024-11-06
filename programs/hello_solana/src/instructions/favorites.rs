use anchor_lang::prelude::*;
use crate::{greetings, state::favorites::Favorites};

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(
        init_if_needed, 
        payer = payer, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, 
        seeds=[b"favorites", payer.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    #[account(mut)] 
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_set_favorites(ctx: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
    greetings!(ctx.program_id);

    let payer_public_key = ctx.accounts.payer.key();
    msg!(
        "User {payer_public_key}'s favorite number is {number}, favorite color is: {color}, and their hobbies are {hobbies:?}",
    );

    ctx.accounts.favorites.set_inner(Favorites {
        number,
        color,
        hobbies
    });
    
    Ok(())
}
