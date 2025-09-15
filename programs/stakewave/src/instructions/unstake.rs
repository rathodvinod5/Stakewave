use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::states::{Pool, UserStakeInfo};
use crate::errors::CustomErrors;

/// Unstake tokens from the pool
/// - Transfers tokens back from the pool vault to the user
/// - Updates reward debt before reducing stake
pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    // Extract accounts to used in the scope
    let users_stake = &mut ctx.accounts.users_stake;
    let user_staking_token_ata = &mut ctx.accounts.user_staking_ata;
    let pool = &mut ctx.accounts.pool;

    // Validate if the user stake is > 0
    require!(users_stake.staked_amount >= amount, CustomErrors::InsufficientStake);

    // Update rewards
    // update_rewards(ctx, amount);

    // Transfer tokens
    let transfer_token_accounts = Transfer {
        from: ctx.accounts.staking_vaut.to_account_info(),
        to: user_staking_token_ata.to_account_info(),
        authority: pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let seeds = [b"pool", pool.staking_mint.as_ref(), pool.reward_mint.as_ref(), &[pool.bump]];
    let signer_seeds= &[&seeds[..]];
    let _ = token::transfer(
        CpiContext::new_with_signer(cpi_program, transfer_token_accounts, signer_seeds), 
    amount);

    // Update state
    users_stake.staked_amount = users_stake.staked_amount.checked_sub(amount).unwrap();
    pool.total_staked = pool.total_staked.checked_sub(amount).unwrap();

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