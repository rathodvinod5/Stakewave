use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::states::{Pool, UserStakeInfo};

pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_staking_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_vaut: Account<'info, TokenAccount>,

    // check if it works, otherwise provide pub key from fron-end/client
    #[account(
        mut,
        seeds = [b"pool", pool.staking_mint.as_ref(), pool.reward_mint.as_ref()],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [b"user-stake", pool.key().as_ref(), user.key().as_ref()],
        bump = users_stake.bump,
    )]
    pub users_stake: Account<'info, UserStakeInfo>,

    pub systme_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}