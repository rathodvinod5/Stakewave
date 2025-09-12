use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub authority: Pubkey,
    pub staking_mint: Pubkey,
    pub reward_mint: Pubkey,
    pub staking_vault: Pubkey,
    pub reward_vaule: Pubkey,
    pub reward_rate:u64,
    pub last_update_time: i64,
    pub acc_reward_per_token: u128,
    pub total_staked: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserStakeInfo {
    pub user: Pubkey,
    pub pool: Pubkey,
    pub staked_amount: u64,
    pub reward_debt: u64,
    pub pending_reward: u64,
    pub bump: u8,
}
