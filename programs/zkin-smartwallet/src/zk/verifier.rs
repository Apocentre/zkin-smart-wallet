use anchor_lang::prelude::*;
use groth16_solana::groth16::{self, Groth16Verifier};
use crate::{
  account_data::zkp::Zkp, program_error::ErrorCode, zk::verifying_key::VERIFYING_KEY
};

pub fn prepare_input(zkp: &mut Zkp) -> Result<()> {
  let prepared_public_inputs = groth16::prepare_inputs(
    &VERIFYING_KEY,
    zkp.prepared_public_inputs,
    zkp.convert_public_inputs(),
    zkp.offset(),
  ).map_err(|_| ErrorCode::InvalidProofData)?;

  zkp.prepared_public_inputs = Some(prepared_public_inputs);
  zkp.next_iteration();

  Ok(())
}


pub fn verify_proof(zkp: &Zkp) -> Result<()> {
  let mut verifier = Groth16Verifier::new(
    &zkp.proof_a,
    &zkp.proof_b,
    &zkp.proof_c,
    zkp.prepared_public_inputs.unwrap(),
    &VERIFYING_KEY,
  ).map_err(|_| ErrorCode::InvalidProofData)?;
  
  verifier.verify().map_err(|_| ErrorCode::GrothVerificationError)?;

  Ok(())
}
