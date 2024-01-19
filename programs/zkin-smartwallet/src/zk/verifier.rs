use anchor_lang::prelude::*;
use groth16_solana::groth16::Groth16Verifier;
use crate::{
  zk::verifying_key::VERIFYING_KEY, program_error::ErrorCode,
};

pub fn verify_proof(
  wallet_address: &str,
  proof_a: [u8; 64],
  proof_b: [u8; 128],
  proof_c: [u8; 64],
  public_inputs_vec: Vec<u8>,
) -> Result<()> {
  // TODO: Extract each public input to a separate variable
  
  // let  public_inputs_vec: [[u8; 32]; NR_INPUTS] = ...s;

  // let mut verifier = Groth16Verifier::new(
  //   &proof_a,
  //   &proof_b,
  //   &proof_c,
  //   &public_inputs_vec,
  //   &VERIFYING_KEY,
  // ).map_err(|_| ErrorCode::InvalidProofData)?;
  
  // verifier.verify().map_err(|_| ErrorCode::GrothVerificationError)?;

  Ok(())
}
