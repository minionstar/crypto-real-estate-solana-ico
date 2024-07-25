use anchor_lang::prelude::*;

#[account]
pub struct ICOState {
    pub remaining_ico_amount: u64, // Remaining amount of ICO tokens
    pub total_sold_usd: u64, // Total amount of fund raised in USD
    pub total_usdt: u64, // Total amount of USDT collected
    pub total_usdc: u64, // Total amount of USDC collected
}

impl ICOState {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8;
}
