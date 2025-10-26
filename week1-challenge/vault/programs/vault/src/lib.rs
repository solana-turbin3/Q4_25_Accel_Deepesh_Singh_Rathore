pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("22grD33JBxV7au5EB2Fc3y4etFkiUt5MwUGM8TMBad1w");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.initialize_vault()
    }
}
