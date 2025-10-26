use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TranferOperation<'info>{
    #[account(
        mut
    )]
    pub owner : Signer<'info>,

    #[account(
        mut,
        seeds = [b"vaultstate",owner.key().as_ref()],
        bump = vault.bump
    )]
    pub vault = 
}