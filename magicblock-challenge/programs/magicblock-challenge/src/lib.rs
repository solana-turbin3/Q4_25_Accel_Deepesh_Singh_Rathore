pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Gmr1hJX9Mqv7wSqVXat5MpsYFPEKZEuD1C356aiAKyJv");

#[program]
pub mod magicblock_challenge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
