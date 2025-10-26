use anchor_lang::prelude::*;

use crate::{state::vault, Vault};

#[derive(Accounts)]
pub struct InitializeVault<'info> {

    #[account(mut)]
    pub admin : Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + 8,
        seeds = [b"vault",admin.key().as_ref()],
        bump
    )]
    pub vault : Account<'info,Vault>,

    pub system_program : Program<'info,System>
}

impl <'info> InitializeVault <'info> {
    pub fn initialize_vault(&mut self)-> Result<()>{
        self.vault.set_inner(Vault { bump: self.vault.bump });
        Ok(())
    }
    
}
