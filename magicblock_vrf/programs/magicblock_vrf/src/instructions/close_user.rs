use anchor_lang::prelude::*;

use crate::User;

#[derive(Accounts)]
pub struct CloseUser<'info> {
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_account_pda : Account<'info,User>,
    pub system_program : Program<'info,System>
}

impl <'info> CloseUser <'info> {
    pub fn close_user(&mut self) -> Result<()> {
    Ok(())
}
    
}
