use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Invalid input")]
    InvalidInput,

    #[msg("Keys are not equal!")]
    KeysNotEqual,

    #[msg("Not have enough token")]
    NotHaveEnoughTokens,

    #[msg("Insufficient staked balance to unstake")] 
    InsufficientStake,

    #[msg("No rewards to claim")]
    NoRewardsToClaim,

    #[msg("Insufficient reward vault balance")]
    InsufficientRewardVaultBalance
}
