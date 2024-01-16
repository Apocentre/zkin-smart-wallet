use anchor_lang::prelude::*;

#[account]
pub struct Wallet {
  /// The owner of this wallet
  pub owner: Pubkey,
}
