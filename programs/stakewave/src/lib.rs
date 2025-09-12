use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod errors;

declare_id!("B3Y9N8wdgQvEXJvTSD8gqdvGeqakEvBFqxY1xMW8XMwS");

#[program]
pub mod stakewave {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
