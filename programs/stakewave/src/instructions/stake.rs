use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

use crate::states::*;

pub fn stake(ctx: Context<Stake>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_staking_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = user,
        space = 8 + UserStakeInfo::INIT_SPACE,
        seeds = [b"user-stake", pool.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub users_stake: Account<'info, UserStakeInfo>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}