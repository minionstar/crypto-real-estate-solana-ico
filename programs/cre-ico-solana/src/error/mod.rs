use anchor_lang::prelude::*;

// NOTE: Anchor 0.27 adds 6000 for user error codes)
// (old Anchor 0.18 added 300 for user error codes)
#[error_code]
pub enum ErrorCodes {
    #[msg("Invalid permission for this method")]
    Unauthorized,

    #[msg("Insufficient USDT for user deposit")]
    InsufficientUserUSDTAmount,

    #[msg("Insufficient USDC for user deposit")]
    InsufficientUserUSDCAmount,

    #[msg("Insufficient SOL for user deposit")]
    InsufficientUserSOLAmount,
}
