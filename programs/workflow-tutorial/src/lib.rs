use anchor_lang::prelude::*;

declare_id!("wocur7QRRMdzPZN52688gBa5iJD4mLkNWSxN5xGGRjY");

#[program]
pub mod workflow_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from version 4: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
