use anchor_lang::prelude::*;

declare_id!("63E3knaFYs7ZSPuDiUkSwmZNAgPoD7J3y9Qiay5otRdZ");

#[program]
pub mod elfo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
