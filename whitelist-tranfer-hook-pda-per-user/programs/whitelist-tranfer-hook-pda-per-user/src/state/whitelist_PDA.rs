use anchor_lang::prelude::*;

#[account]
pub struct WhitelistPDA{
   pub is_whitelisted : bool,
   pub bump : u8
}