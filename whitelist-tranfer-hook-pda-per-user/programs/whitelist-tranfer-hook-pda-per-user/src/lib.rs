pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("33s6C77xjRFwnJLfK3EqdP4CSLEek68vYUiJxcaXj7hU");

#[program]
pub mod whitelist_tranfer_hook_pda_per_user {

    use super::*;
    pub fn initialize(ctx: Context<CreateWhitelistPDA>) -> Result<()> {
        ctx.accounts.create_whitelist_pda()
    }
    pub fn close(ctx: Context<CloseWhitelistPDA>)-> Result<()>{
        ctx.accounts.close_whitelist_pda()
    }
}
