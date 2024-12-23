use anchor_lang::prelude::*;

declare_id!("43z1aSKYnKWWRnakYsxXjqEJ9WsGBHSabLxMhTDxDxWX");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
