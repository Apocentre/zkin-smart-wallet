use anchor_lang::prelude::*;
use crate::account_data::wallet::{wallet_size, Wallet};

#[derive(Accounts)]
#[instruction(wallet_address: [u8; 32])]
pub struct CreateWallet<'info> {
  /// CHECK: The PDA that represent the actial wallet
  #[account(
    init,
    payer = owner,
    space = wallet_size(),
    seeds = [wallet_address.as_ref()],
    bump,
  )]
  pub wallet: Account<'info, Wallet>,
  
  #[account(mut)]
  pub owner: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
