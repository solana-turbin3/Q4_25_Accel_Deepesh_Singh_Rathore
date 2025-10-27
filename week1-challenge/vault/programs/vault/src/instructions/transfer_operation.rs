use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
    associated_token::AssociatedToken,
};
use spl_transfer_hook_interface::onchain::add_extra_accounts_for_execute_cpi;

use crate::Vault;

#[derive(Accounts)]
pub struct TranferOperation<'info>{
    #[account(
        mut
    )]
    pub admin : Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault",admin.key().as_ref()],
        bump
    )]
    pub vault : Account<'info,Vault>,

    #[account(
        
    )]
    pub mint : InterfaceAccount<'info,Mint>,

    pub system_program : Program<'info,System>
}



