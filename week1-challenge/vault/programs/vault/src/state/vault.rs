use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub bump : u8
}

#[account]
pub struct Vault{}