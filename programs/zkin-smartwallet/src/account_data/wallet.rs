use std::mem::size_of;
use anchor_lang::prelude::*;

#[account]
pub struct Wallet {
  /// The ZkIn address of this wallet where `address = H(sub|iss|aud|salt)`
  /// The address is base58 encoded value of the above hash
  pub address: String,
  /// The owner of this wallet
  pub owner: Pubkey,
  /// PDA bump
  pub bump: u8,
}

impl Wallet {
  pub fn size() -> usize {
    // 8 discriminator
    // size_of wallet which includes size of Pubkey a String fat pointer and the u8 bump.
    // 64 actual address string length
    8 + size_of::<Wallet>() + 64
  }
}
