mod verifying_key;
mod account_data;
mod instructions;
mod processors;
mod program_error;

use anchor_lang::prelude::*;
use crate::{
  processors::create_wallet::NR_INPUTS,
  instructions::{
    create_wallet::*, verify_jwt::*,
  },
};

declare_id!("zkinKSHW3PijK2ZyRDUSXe8m2UKnNjJPvnv2NeZUvHy");

#[program]
pub mod zkin_smartwallet {
  use super::*;

  pub fn create_wallet(
    ctx: Context<CreateWallet>,
    wallet_address: String,
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    public_inputs_vec: [[u8; 32]; NR_INPUTS],
  ) -> Result<()> {
    processors::create_wallet::exec(ctx, wallet_address, proof_a, proof_b, proof_c, public_inputs_vec)
  }

  pub fn exec(
    ctx: Context<VerifyJwt>,
    header: Vec<u8>,
    payload: Vec<u8>,
    sig: Vec<u8>,
  ) -> Result<()> {
    processors::verify_jwt::exec(ctx, header, payload, sig)
  }
}
