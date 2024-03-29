mod zk;
mod account_data;
mod instructions;
mod processors;
mod program_error;
mod constants;

use anchor_lang::prelude::*;
use crate::{
  instructions::{
    create_wallet::*, init_zkp::*, prepare_zkp::*, initialize::*, update_operators::*,
    register_rsa_modulus::*,
  },
  account_data::{
    zkp::PUBLIC_INPUTS_LEN, state::MAX_OPERATORS,
  },
};

declare_id!("zkinKSHW3PijK2ZyRDUSXe8m2UKnNjJPvnv2NeZUvHy");

#[program]
pub mod zkin_smartwallet {
  use account_data::state::MAX_OPERATORS;

use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    processors::initialize::exec(ctx)
  }

  /// Updates the list of operators
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `operators` - The list of the new operators
  pub fn update_operators(ctx: Context<UpdateOperators>, operators: [Pubkey; MAX_OPERATORS]) -> Result<()> {
    processors::update_operators::exec(ctx, operators)
  }

  /// Registers a new RSA modulus for the given auth provider. The first time it's called it will create a new
  /// AuthProvider PDA account.
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `provider` - The auth provider which is the same as the iss claim value
  /// * `operrsa_modulusators` - The new rsa_modulus to register for the given auth provider
  pub fn register_rsa_modulus(
    ctx: Context<RegisterRsaModulus>,
    _provider: String,
    rsa_modulus: [u8; 32],
  ) -> Result<()> {
    processors::register_rsa_modulus::exec(ctx, rsa_modulus)
  }

  /// Currently the Solana runtime has heap size limit of 32Kb per transaction. Our public inputs have a length
  /// of 249 bytes which are converted into a Circo Field and each byte becomes [u8; 32]. The Groth16 verifier
  /// use `alt_bn128`` to verify the ZKP. That library uses vectors and allocates more that 32 KB of heap memory.
  /// More on this here https://github.com/anza-xyz/agave/issues/356
  /// To circumvent this, we split the verification process into pre-processing. Each pre-process works on a section
  /// of the public inputs (big enough to keep the heap size within the limits). The first step to run the pre-process
  /// is to call this instruction which will store the ZKP and the subsequent instruction will use the data store in
  /// this ix.
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
  /// * `public_inputs` - All public inputs to the circuit. The len is calculated as so:
  /// iss_out + aud_out + nonce_out + exp_out + wallet_address + rsa_modulo = 78 + 78 + 78 + 10 + 32 + 32
  /// Note the wallet address and the rsa_modulo which are hex encoded values
  pub fn init_zkp(
    ctx: Context<InitZkp>,
    _wallet_address: [u8; 32],
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    public_inputs: [u8; PUBLIC_INPUTS_LEN],
    batch_size: u8,
  ) -> Result<()> {
    processors::init_zkp::exec(
      ctx,
      proof_a,
      proof_b,
      proof_c,
      public_inputs,
      batch_size,
    )
  }

  /// Runs each iteration of the public input procesing. The concept is described in the documentation of `init_zkp` above
  /// 
  /// # Arguments
  /// 
  /// * `ctx` - The Anchor context holding the accounts
  pub fn prepare_zkp(ctx: Context<PrepareZkp>, _wallet_address: [u8; 32]) -> Result<()> {
    processors::prepare_zkp::exec(ctx)
  }

  /// Allows anyone to provide a valid ZKP and create a new smart wallet
  ///
  /// # Arguments
  /// 
  /// * `ctx` - The Anchor context holding the accounts
  /// * `wallet_address` - This is a deterministic address which is `address = H(sub|iss|aud|salt)` where H = Poseidon
  /// * `provider` - The auth provider which is the same as the iss claim value
  pub fn create_wallet(
    ctx: Context<CreateWallet>,
    wallet_address: [u8; 32],
    provider: String,
    _test_ts: i64,
  ) -> Result<()> {
    #[cfg(not(feature = "localnet"))]
    let now = Clock::get().unwrap().unix_timestamp;
    #[cfg(feature = "localnet")]
    let now = _test_ts;

    processors::create_wallet::exec(ctx, wallet_address, provider, now)
  }
}
