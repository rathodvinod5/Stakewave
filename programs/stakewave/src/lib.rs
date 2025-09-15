use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;
pub mod states;
pub mod errors;

declare_id!("B3Y9N8wdgQvEXJvTSD8gqdvGeqakEvBFqxY1xMW8XMwS");

#[program]
pub mod stakewave {
    use super::*;

    pub fn initialize(ctx: Context<InitializePool>, reward_rate: u64) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        initialize_pool(ctx, reward_rate);
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake(ctx, amount);
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        claim(ctx);
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        unstake(ctx);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
