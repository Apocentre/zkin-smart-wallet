use anchor_lang::prelude::*;
use groth16_solana::groth16;
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


// pub fn verify_proof(
//   proof_a: [u8; 64],
//   proof_b: [u8; 128],
//   proof_c: [u8; 64],
//   public_inputs: [u8; 308],
// ) -> Result<()> {
//   let pub_inputs = convert_public_inputs(public_inputs);

//   let mut verifier = Groth16Verifier::<246>::new(
//     &proof_a,
//     &proof_b,
//     &proof_c,
//     &pub_inputs,
//     &VERIFYING_KEY,
//   ).map_err(|_| ErrorCode::InvalidProofData)?;
  
//   verifier.verify().map_err(|_| ErrorCode::GrothVerificationError)?;

//   Ok(())
// }
