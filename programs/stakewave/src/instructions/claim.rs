use anchor_lang::prelude::*;
use anchor_lang::solana_program::example_mocks::solana_transaction::Transaction;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::states::{Pool, UserStakeInfo};
use crate::errors::CustomErrors;

pub fn claim(ctx: Context<Claim>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let users_stake = &mut ctx.accounts.users_stake;
    let user_reward_token_ata = &mut ctx.accounts.user_reward_token_ata;
    let reward_vault = &mut ctx.accounts.reward_vault;

    // update_rewards(ctx, pool)

    // check if reward is not empty or zero
    let reward_amount = users_stake.pending_reward;
    require!(reward_amount > 0, CustomErrors::InsufficientStake);
    // Ensure pool has enough reward balance
    require!(reward_vault.amount >= reward_amount, CustomErrors::InsufficientRewardVaultBalance);

    users_stake.pending_reward = 0;

    let transfer_accounts = Transfer {
        from: reward_vault.to_account_info(),
        to: user_reward_token_ata.to_account_info(),
        authority: pool.to_account_info()
    };

    let token_program = ctx.accounts.token_program.to_account_info();
    let seeds = [b"pool", pool.staking_mint.as_ref(), pool.reward_mint.as_ref(), &[pool.bump]];
    let signer_seeds = &[&seeds[..]];

    let _ = token::transfer(
        CpiContext::new_with_signer(
            token_program, 
            transfer_accounts, 
            signer_seeds
        ), 
    reward_amount);

    Ok(())
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // User's reward token account
    #[account(mut)]
    pub user_reward_token_ata: Account<'info, TokenAccount>,

    // Pool reward vault
    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,

    // Pool state
    // #[account(mut)]
    #[account(
        mut,
        seeds = [b"pool", pool.staking_mint.key().as_ref(), pool.reward_mint.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    // User stake state
    #[account(
        mut,
        seeds = [b"user-stake", pool.key().as_ref(), user.key().as_ref()],
        bump = users_stake.bump
    )]
    pub users_stake: Account<'info, UserStakeInfo>,

    // pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}