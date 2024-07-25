use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Transfer };

use crate::state::*;
// use crate::error::*;

// pub fn deposit_checks_and_effects(
//     presale: &mut Account<ICOInfo>,
//     amount: u64,
//     is_stable_token: bool,
//     sol_price: u64
// ) -> Result<(u64, u64)> {
// let stage_iterator = presale.stage_iterator as usize;
// require!(
//     presale.stages[stage_iterator].token_amount >= amount,
//     ErrorCodes::InsufficientStageTokens
// );

// let stage = &mut presale.stages[stage_iterator];
// let coin_price = if is_stable_token { 1 } else { sol_price };

// let expected_amount = (coin_price * (amount as u64)) / (stage.token_price as u64);
//     let expected_amount = 0; // Placeholder for actual charge back calculation
//     let charge_back = 0; // Placeholder for actual charge back calculation

//     Ok((charge_back as u64, expected_amount as u64))
// }

pub fn transfer_tokens<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64
) -> Result<()> {
    let cpi_accounts = Transfer {
        from,
        to,
        authority,
    };
    let cpi_program = token_program;
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)
}

// pub fn update_presale_state(presale: &mut Account<ICOInfo>, amount: u64, price: u64, user: Pubkey) {
// let stage_iterator = presale.stage_iterator as usize;
// let stage = &mut presale.stages[stage_iterator];
// stage.token_amount -= amount;
// presale.total_tokens_sold += amount;
// presale.total_sold_in_usd += price * amount;

// let balance = presale.balances.entry(user).or_insert(0);
// *balance += amount;

// if stage.token_amount == 0 {
//     presale.stage_iterator += 1;
// }
// }

// pub fn get_coin_price(coin_price_feed: Pubkey) -> Result<u64> {
//     // Placeholder function to fetch the coin price
//     // Replace with actual logic to fetch the coin price from a price feed
//     Ok(100_000_000) // Example value
// }

pub fn get_precision_factor(ico_info: &Account<ICOInfo>) -> u64 {
    let base: u64 = 10;
    let precision_decimals = ico_info.ico_token_mint_decimals - STABLECOIN_DECIMALS;
    let precision_factor = base.pow(precision_decimals as u32);
    precision_factor
}
