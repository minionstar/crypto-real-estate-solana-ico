use anchor_lang::prelude::*;

#[event]
pub struct Deposit {
    pub staker: Pubkey,
    pub amount: u64,
}
