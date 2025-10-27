use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{anchor::delegate,cpi::DelegateConfig};

use crate::User;

#[delegate]
#[derive(Accounts)]

pub struct Delegate<'info>{

    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        mut,
        del,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account_pda.bump
    )]
    pub user_account_pda : Account<'info,User>,

    /// CHECK :
    pub validator : UncheckedAccount<'info>,
    pub system_program : Program<'info,System>
}

impl <'info> Delegate<'info> {

    pub fn delegate(&mut self)->Result<()>{
        
        let pda_seeds : &[&[u8]] = &[
            b"user",
            &self.user.key.as_ref(),
        ];

        self.delegate_user_account_pda(
            &self.user,
            pda_seeds,
            DelegateConfig {
                validator: Some(self.validator.key()),
                ..Default::default()
            },
        )?;
        Ok(())
    }
}

