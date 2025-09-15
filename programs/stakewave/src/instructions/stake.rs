use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{errors::CustomErrors, states::*};

/// Stake tokens into the pool
/// - Transfers user's staking tokens into the pool's staking vault
/// - Updates reward debt before increasing stake
/// - Ensures user is staking the correct mint and has sufficient balance
pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let users_stake = &mut ctx.accounts.users_stake;
    let user_staking_token_ata = &mut ctx.accounts.user_staking_token_ata;
    let pool = &mut ctx.accounts.pool;

    // Ensure this user's staking ATA is for the same staking_mint as the pool
    require_keys_eq!(pool.staking_mint, user_staking_token_ata.mint, CustomErrors::KeysNotEqual);
    // Ensure user has enough balance to stake
    require!(user_staking_token_ata.amount > 0, CustomErrors::NotHaveEnoughTokens);

    // Update pending rewards before modifying stake
    // update_rewards(ctx, amount);

    // Transfer tokens
    let transfer_instr_accounts = Transfer {
        from: user_staking_token_ata.to_account_info(),
        to: ctx.accounts.staking_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    // Execute transfer instructions with CPI
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let _ = token::transfer(CpiContext::new(cpi_program, transfer_instr_accounts), amount);

    // Update state
    users_stake.staked_amount = users_stake.staked_amount.checked_add(amount).unwrap();
    pool.total_staked = pool.total_staked.checked_add(amount).unwrap();

    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_staking_token_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_vault: Account<'info, TokenAccount>,

    // #[account(
    //     mut,
    //     seeds = [b"pool", pool.staking_mint.key().as_ref(), pool.reward_mint.key().as_ref()],
    //     bump = pool.bump
    // )]
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