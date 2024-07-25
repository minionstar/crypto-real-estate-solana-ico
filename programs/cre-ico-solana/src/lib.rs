use anchor_lang::prelude::*;

mod instructions;
mod state;
mod error;
mod utils;

use instructions::*;

declare_id!("9X5si3xhU4nFVh7FkGaC3n251xoN5JBoys9AEnrfkzxh");

#[program]
mod cre_ico_solana {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        ico_name: String,
        ico_amount: u64,
        token_per_usd: u64,
        bump: u8
    ) -> Result<()> {
        instructions::initialize::handler(ctx, ico_name, ico_amount, token_per_usd, bump)
    }

    pub fn deposit_usdt(ctx: Context<DepositUSDT>, usdt_amount: u64) -> Result<()> {
        instructions::deposit_usdt::handler(ctx, usdt_amount)
    }

    pub fn withdraw_usdt(ctx: Context<WithdrawUSDT>) -> Result<()> {
        instructions::withdraw_usdt::handler(ctx)
    }
}
