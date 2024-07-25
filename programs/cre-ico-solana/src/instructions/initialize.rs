use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, TokenAccount };

use crate::{ state::*, utils::transfer_tokens };

pub fn handler(
    ctx: Context<Initialize>,
    ico_name: String,
    ico_amount: u64,
    token_per_usd: u64,
    bump: u8
) -> Result<()> {
    let ico_info = &mut ctx.accounts.ico_info;
    let ico_state = &mut ctx.accounts.ico_state;

    let name_bytes = ico_name.as_bytes();
    let mut name_data = [b' '; 10];
    name_data[..name_bytes.len()].copy_from_slice(name_bytes);

    ico_info.ico_name = name_data;
    ico_info.authority = ctx.accounts.authority.key();
    ico_info.total_ico_amount = ico_amount;
    ico_info.token_per_usd = token_per_usd;

    ico_info.ico_token_mint = ctx.accounts.ico_token_mint.key();
    ico_info.usdt_mint = ctx.accounts.usdt_mint.key();
    ico_info.usdc_mint = ctx.accounts.usdc_mint.key();
    ico_info.ico_token_mint_decimals = ctx.accounts.ico_token_mint.decimals;
    ico_info.bump = bump;

    ico_state.remaining_ico_amount = ico_amount;
    ico_state.total_sold_usd = 0;
    ico_state.total_usdt = 0;
    ico_state.total_usdc = 0;

    let _ = transfer_tokens(
        ctx.accounts.admin_ico_token_account.to_account_info(),
        ctx.accounts.protocol_ico_token_pda.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ico_amount
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(ico_name: String, bump: u8)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = ICOInfo::LEN, seeds = [ico_name.as_bytes()], bump)]
    pub ico_info: Box<Account<'info, ICOInfo>>,

    #[account(init, payer = authority, space = ICOState::LEN, seeds = [b"ico_state"], bump)]
    pub ico_state: Box<Account<'info, ICOState>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = admin_ico_token_account.owner == authority.key(),
        constraint = admin_ico_token_account.mint == ico_token_mint.key()
    )]
    pub admin_ico_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = authority,
        token::mint = ico_token_mint,
        token::authority = ico_info,
        seeds = [b"protocol_ico_token"],
        bump
    )]
    pub protocol_ico_token_pda: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = authority,
        token::mint = usdt_mint,
        token::authority = ico_info,
        seeds = [b"protocol_usdt_pool"],
        bump
    )]
    pub protocol_usdt_pool_pda: Box<Account<'info, TokenAccount>>,

    #[account(constraint = ico_token_mint.key() == admin_ico_token_account.mint)]
    pub ico_token_mint: Box<Account<'info, Mint>>,

    #[account(constraint = usdt_mint.decimals == STABLECOIN_DECIMALS)]
    pub usdt_mint: Box<Account<'info, Mint>>,

    #[account(constraint = usdc_mint.decimals == STABLECOIN_DECIMALS)]
    pub usdc_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, token::Token>,
}
