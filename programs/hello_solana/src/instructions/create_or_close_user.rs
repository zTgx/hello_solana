use anchor_lang::prelude::*;
use crate::{greetings, state::UserState};

#[derive(Accounts)]
pub struct CreateUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserState::INIT_SPACE,
        seeds = [
            b"USER",
            user.key().as_ref(),
        ],
        bump
    )]
    pub user_account: Account<'info, UserState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"USER",
            user.key().as_ref(),
        ],
        bump = user_account.bump,
        close = user, // close account and return lamports to user
    )]

    pub user_account: Account<'info, UserState>,
}

pub fn handle_create_user(ctx: Context<CreateUserContext>, name: String) -> Result<()> {
    greetings!(ctx.program_id);

    *ctx.accounts.user_account = UserState {
        bump: ctx.bumps.user_account,
        user: ctx.accounts.user.key(),
        name,
    };

    Ok(())
}

pub fn handle_close_user(ctx: Context<CloseUserContext>) -> Result<()> {
    greetings!(ctx.program_id);

    Ok(())
}
