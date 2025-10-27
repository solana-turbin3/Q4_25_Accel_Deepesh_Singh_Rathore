use anchor_lang::prelude::*;

use crate::User;

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account_pda.bump
    )]
    pub user_account_pda : Account<'info,User>,
    pub system_program : Program<'info,System>
}

impl <'info> UpdateUser <'info> {
    pub fn update_user(&mut self, new_data : u8) -> Result<()> {
        self.user_account_pda.random_number = new_data;
    Ok(())
}
    
}
