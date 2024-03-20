mod zk;
mod account_data;
mod instructions;
mod processors;
mod program_error;

use anchor_lang::prelude::*;
use crate::instructions::create_wallet::*;

declare_id!("zkinKSHW3PijK2ZyRDUSXe8m2UKnNjJPvnv2NeZUvHy");

#[program]
pub mod zkin_smartwallet {
  use super::*;

  /// Allows anyone to provide a valid ZKP and create a new smart wallet
  ///
  /// # Arguments
  /// 
  /// * `ctx` - The Anchor context holding the accounts
  /// * `wallet_address` - This is a deterministic address which is `address = H(sub|iss|aud|salt)` where H = Poseidon
  /// * `proof_a` - Part of the ZKP
  /// * `proof_b` - Part of the ZKP
  /// * `proof_c` - Part of the ZKP
  /// * `public_inputs_vec` - All public inputs except for address and rsa_modulus which are passed separately
  /// * `address` - The 
  pub fn create_wallet(
    ctx: Context<CreateWallet>,
    wallet_address: String,
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    public_inputs_vec: [u8; 244],
    address: Vec<[u8; 32]>,
    rsa_modulus: Vec<[u8; 32]>,
  ) -> Result<()> {
    processors::create_wallet::exec(
      ctx,
      wallet_address,
      proof_a,
      proof_b,
      proof_c,
      public_inputs_vec,
      address,
      rsa_modulus,
    )
  }
}
