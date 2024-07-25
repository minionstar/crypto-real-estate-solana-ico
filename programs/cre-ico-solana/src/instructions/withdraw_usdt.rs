use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, TokenAccount, Transfer };

use crate::state::*;

pub fn handler(ctx: Context<WithdrawUSDT>) -> Result<()> {
    // Transfer USDT from protocol to admin
    // let user_balance = ctx.accounts.user_usdt_token_account.to_account_info().lamports();
    // require!(user_balance > usdt_amount, ErrorCodes::InsufficientUserUSDTAmount);

    let ico_info = &mut ctx.accounts.ico_info;
    let bump = ico_info.bump;
    let seeds = &[b"cre_ico".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.protocol_usdt_pool_pda.to_account_info(),
        to: ctx.accounts.admin_usdt_token_account.to_account_info(),
        authority: ctx.accounts.ico_info.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    let _ = token::transfer(cpi_ctx, ctx.accounts.protocol_usdt_pool_pda.amount);

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawUSDT<'info> {
    #[account(seeds = [b"cre_ico"], bump = ico_info.bump)]
    pub ico_info: Account<'info, ICOInfo>,

    #[account(mut)]
    pub ico_state: Account<'info, ICOState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        // constraint = admin_usdt_token_account.owner == authority.key(),
        // constraint = admin_usdt_token_account.mint == usdt_mint.key(),
    )]
    pub admin_usdt_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"protocol_usdt_pool"],
        bump
    )]
    pub protocol_usdt_pool_pda: Box<Account<'info, TokenAccount>>,

    #[account(constraint = usdt_mint.decimals == STABLECOIN_DECIMALS)]
    pub usdt_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, token::Token>,
}
