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
  /// It's a hex encoded bytes value. It should be part of the public_inputs_vec as well. But we pass it one more time as
  /// a separate param so we can easily create the wallet PDA which the wallet_address is the seed of.
  /// * `proof_a` - Part of the ZKP
  /// * `proof_b` - Part of the ZKP
  /// * `proof_c` - Part of the ZKP
  /// * `public_inputs_vec` - All public inputs to the circuit. The len is calculated as so:
  /// iss_out + aud_out + nonce_out + exp_out + wallet_address + rsa_modulo = 78 + 78 + 78 + 10 + 32 + 32
  /// Note the wallet address and the rsa_modulo which are hex encoded values
  pub fn create_wallet(
    ctx: Context<CreateWallet>,
    _wallet_address: [u8; 32],
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    public_inputs_vec: [u8; 308],
  ) -> Result<()> {
    processors::create_wallet::exec(
      ctx,
      proof_a,
      proof_b,
      proof_c,
      public_inputs_vec,
    )
  }
}
