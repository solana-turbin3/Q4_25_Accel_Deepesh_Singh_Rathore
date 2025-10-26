use anchor_lang::prelude::*;

declare_id!("cv2EkBy4VXUfHxSkg7cDdNwT1GsFTZaqXiYDtXR6fST");

#[program]
pub mod t_hook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
