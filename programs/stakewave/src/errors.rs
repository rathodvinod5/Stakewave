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
    NotHaveEnoughTokens
}
