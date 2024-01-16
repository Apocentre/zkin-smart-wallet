use anchor_lang::prelude::*;
use groth16_solana::groth16::Groth16Verifier;
use crate::{
  verifying_key::VERIFYING_KEY,
  program_error::ErrorCode,
  instructions::create_wallet::CreateWallet,
};

pub const NR_INPUTS: usize = 10;

pub fn exec(
  ctx: Context<CreateWallet>,
  wallet_address: String,
  proof_a: [u8; 64],
  proof_b: [u8; 128],
  proof_c: [u8; 64],
  public_inputs_vec: [[u8; 32]; NR_INPUTS],
) -> Result<()> {
  let mut verifier = Groth16Verifier::<NR_INPUTS>::new(
    &proof_a,
    &proof_b,
    &proof_c,
    &public_inputs_vec,
    &VERIFYING_KEY,
  ).map_err(|_| ErrorCode::InvalidProofData)?;
  
  verifier.verify().map_err(|_| ErrorCode::GrothVerificationError)?;

  // update wallet state
  let wallet = &mut ctx.accounts.wallet;
  wallet.address = wallet_address;
  wallet.owner = ctx.accounts.owner.key();
  wallet.bump = ctx.bumps.wallet;

  Ok(())
}
