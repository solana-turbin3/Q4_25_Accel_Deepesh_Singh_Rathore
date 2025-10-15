use anchor_lang::prelude::*;

declare_id!("Aa35VaonfZa49Ke2UDoXtAv9axHRZf4wfDLWYh6puhGE");

#[program]
pub mod random {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
