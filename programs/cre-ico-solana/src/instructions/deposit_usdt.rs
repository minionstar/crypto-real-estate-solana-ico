use anchor_lang::prelude::*;
use anchor_spl::token::{ self, TokenAccount, Transfer };

use crate::error::ErrorCodes;
use crate::state::*;
use crate::utils::get_precision_factor;

pub fn handler(ctx: Context<DepositUSDT>, usdt_amount: u64) -> Result<()> {
    let ico_info = &mut ctx.accounts.ico_info;
    let ico_state = &mut ctx.accounts.ico_state;

    // Transfer USDT from user to protocol
    let user_balance = ctx.accounts.user_usdt_token_account.to_account_info().lamports();
    require!(user_balance > usdt_amount, ErrorCodes::InsufficientUserUSDTAmount);

    let cpi_accounts = Transfer {
        from: ctx.accounts.user_usdt_token_account.to_account_info(),
        to: ctx.accounts.protocol_usdt_pool_pda.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    let _ = token::transfer(cpi_ctx, usdt_amount);

    let transfer_ico_amount = usdt_amount * ico_info.token_per_usd * get_precision_factor(ico_info);
    msg!("transfer_ico_amount: {}", transfer_ico_amount);

    // Transfer ICO tokens from protocol to user
    // let ico_name = ico_info.ico_name;
    let bump = ico_info.bump;
    let seeds = &[b"cre_ico".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.protocol_ico_token_pda.to_account_info(),
        to: ctx.accounts.user_ico_token_account.to_account_info(),
        authority: ctx.accounts.ico_info.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    let _ = token::transfer(cpi_ctx, transfer_ico_amount);

    // Update ICO state
    ico_state.remaining_ico_amount -= transfer_ico_amount as u64;
    ico_state.total_sold_usd += usdt_amount;
    ico_state.total_usdt += usdt_amount;

    Ok(())
}

#[derive(Accounts)]
pub struct DepositUSDT<'info> {
    #[account(seeds = [b"cre_ico"], bump = ico_info.bump)]
    pub ico_info: Account<'info, ICOInfo>,

    #[account(mut)]
    pub ico_state: Account<'info, ICOState>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_usdt_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"protocol_usdt_pool"],
        bump
    )]
    pub protocol_usdt_pool_pda: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_ico_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"protocol_ico_token"],
        bump
    )]
    pub protocol_ico_token_pda: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, token::Token>,
}
