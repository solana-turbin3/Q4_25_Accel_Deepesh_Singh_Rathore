use anchor_lang::prelude::*;

declare_id!("76Cuebsno8uzjKYcG1Bfp23SjemYMe44zRPJuXbuFUnz");

#[program]
pub mod er {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
