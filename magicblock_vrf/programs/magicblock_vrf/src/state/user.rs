use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
   pub random_number : u8,
   pub bump : u8
}

