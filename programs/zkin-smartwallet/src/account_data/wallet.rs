use anchor_lang::prelude::*;

#[account]
pub struct Wallet {
  /// The ZkIn address of this wallet where `address = H(sub|iss|aud|salt)`
  pub address: String,
  /// The owner of this wallet
  pub owner: Pubkey,
  /// PDA bump
  pub bump: u8,
}
