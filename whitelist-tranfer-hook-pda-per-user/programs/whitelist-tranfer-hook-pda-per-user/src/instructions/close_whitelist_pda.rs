use anchor_lang::prelude::*;

use crate::WhitelistPDA;

#[derive(Accounts)]
pub struct CloseWhitelistPDA<'info> {

    #[account(mut)]
    pub admin : Signer<'info>,

    ///CHECK : This is safe because we are only using pubkey. 
    pub target_address :UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"whitelist",target_address.key().as_ref()],
        close = admin,
        bump = whiltelist_pda.bump
    )]
    pub whiltelist_pda : Account<'info,WhitelistPDA>,
    pub system_program : Program<'info,System>
}

impl<'info> CloseWhitelistPDA<'info>{
    pub fn close_whitelist_pda(&mut self)->Result<()>{
        // self.whiltelist_pda.is_whitelisted = false;
        Ok(())
    }
}