use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::states::Pool;

/// Initializes a new staking pool
/// - Creates the `Pool` account (PDA)
/// - Creates staking and reward vault token accounts (owned by the pool PDA)
/// - Sets reward rate and initializes counters
pub fn initialize_pool(ctx: Context<InitializePool>, reward_rate: u64) ->  Result<()> {
    let pool = &mut ctx.accounts.pool;

    pool.authority = ctx.accounts.authority.key();
    pool.staking_mint = ctx.accounts.staking_mint.key();
    pool.reward_mint = ctx.accounts.reward_mint.key();
    pool.staking_vault = ctx.accounts.reward_vault.key();
    pool.reward_vault = ctx.accounts.reward_vault.key();
    pool.reward_rate = reward_rate;
    pool.last_update_time = Clock::get()?.unix_timestamp;
    pool.total_staked = 0;
    pool.acc_reward_per_token = 0;

    Ok(())
}


#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // mint token type to be staked
    pub staking_mint: Account<'info, Mint>,
    // reward token type to be rewarded
    pub reward_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = 8 + Pool::INIT_SPACE,
        seeds = [b"pool", staking_mint.key().as_ref(), reward_mint.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = authority,
        token::mint = staking_mint,
        token::authority = pool,
    )]
    pub staking_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = reward_mint,
        token::authority = pool,
    )]
    pub reward_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}