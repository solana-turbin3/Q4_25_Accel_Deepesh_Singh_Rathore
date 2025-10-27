use anchor_lang::prelude::*;

use crate::User;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"user", user.key().as_ref()],
        space = 8 + User::INIT_SPACE,
        bump
    )]
    pub user_account_pda : Account<'info,User>,
    pub system_program : Program<'info,System>
}

impl <'info> InitializeUser <'info> {
    pub fn initialize_user(&mut self, bumps : &InitializeUserBumps) -> Result<()> {
        self.user_account_pda.random_number = 0;
        self.user_account_pda.bump =  bumps.user_account_pda;
    Ok(())
}
    
}
