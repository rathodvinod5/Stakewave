use anchor_lang::prelude::*;

use crate::states::{Pool, UserStakeInfo};

// Updates reward tracking for a given user
// - Brings pool's acc_reward_per_token up to date based on elapsed time
// - Calculates user's pending rewards
pub fn update_rewards(users_stake: &mut UserStakeInfo, pool: &mut Pool) -> Result<()> {
    let time_now = Clock::get()?.unix_timestamp;
    let duration = time_now - pool.last_update_time;

    if duration > 0 && pool.total_staked > 0 {
        // Calculate total rewards accrued since last update
        let reward = (duration as u64).checked_mul(pool.reward_rate).unwrap();

        // Increase acc_reward_per_token proportionally
        pool.acc_reward_per_token = pool
            .acc_reward_per_token
            .checked_add(reward.checked_div(pool.total_staked).unwrap())
            .unwrap();
        pool.last_update_time = time_now;
    }

    // Calculate user's accumulated rewards based on updated pool state
    let accumalated_reward = users_stake.staked_amount.checked_mul(pool.acc_reward_per_token).unwrap();
    let pending = accumalated_reward.checked_sub(users_stake.reward_debt).unwrap();

    users_stake.pending_reward = users_stake.pending_reward.checked_add(pending).unwrap();
    users_stake.reward_debt = accumalated_reward;

    Ok(())
}