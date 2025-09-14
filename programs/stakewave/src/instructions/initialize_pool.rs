use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::states::*;

pub fn initilize_pool(ctx: Context<InitiliazePool>, reward_rate: u64) ->  Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct InitiliazePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // mint account to be staked
    pub staking_mint: Account<'info, Mint>,
    // mint account as rewarded
    pub reward_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = 8 + Pool::INIT_SPACE,
        seeds = [b"pool", authority.key.as_ref(),],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = authority,
        token::mint = staking_mint,
        token::authority = pool,
    )]
    pub staking_valut: Account<'info, TokenAccount>,

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