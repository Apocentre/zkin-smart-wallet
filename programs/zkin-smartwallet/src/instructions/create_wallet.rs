use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::account_data::wallet::Wallet;

#[derive(Accounts)]
#[instruction(wallet_address: [u8; 32])]
pub struct CreateWallet<'info> {
  /// CHECK: The PDA that represent the actial wallet
  #[account(
    init,
    payer = owner,
    space = 8 + size_of::<Wallet>(),
    seeds = [wallet_address.as_ref()],
    bump,
  )]
  pub wallet: Account<'info, Wallet>,
  
  #[account(mut)]
  pub owner: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
