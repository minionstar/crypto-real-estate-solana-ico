use anchor_lang::prelude::*;

#[account]
pub struct ICOInfo {
    pub ico_name: [u8; 10],
    pub bump: u8, // Bump seed for PDA
    pub authority: Pubkey, // PDA authority for managing the ICO

    pub token_per_usd: u64, // Number of ICO tokens per SOL
    pub total_ico_amount: u64, // Total amount of ICO tokens allocated

    pub ico_token_mint: Pubkey,
    pub usdt_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub ico_token_mint_decimals: u8,
}

impl ICOInfo {
    pub const LEN: usize = 8 + 10 + 1 + 32 + 8 + 8 + 32 + 32 + 32 + 1;
}
