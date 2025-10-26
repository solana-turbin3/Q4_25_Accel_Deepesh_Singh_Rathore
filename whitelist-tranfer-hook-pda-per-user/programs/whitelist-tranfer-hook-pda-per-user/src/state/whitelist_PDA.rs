use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct WhitelistPDA{
   pub is_whitelisted : bool,
   pub bump : u8
}