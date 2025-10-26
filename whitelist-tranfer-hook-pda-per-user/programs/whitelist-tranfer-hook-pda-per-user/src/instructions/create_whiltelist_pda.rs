use anchor_lang::prelude::*;

use crate::whitelist_PDA::WhitelistPDA;

#[derive(Accounts)]
pub struct CreateWhitelistPDA<'info> {

    #[account(mut)]
    pub admin : Signer<'info>,

    ///CHECK : This is safe because we are only using pubkey. 
    pub target_address :UncheckedAccount<'info>,

    #[account(
        init,
        payer = admin,
        space = 8,
        seeds = [b"whiltelist",target_address.key().as_ref()],
        bump
    )]
    pub whiltelist_pda : Account<'info,WhitelistPDA>,
    pub system_program : Program<'info,System>
}

impl<'info> CreateWhitelistPDA<'info>{
    pub fn create_whitelist_pda(&mut self)->Result<()>{
        self.whiltelist_pda.set_inner(WhitelistPDA{
            is_whitelisted : true,
            bump : self.whiltelist_pda.bump
        });
        Ok(())
    }
}