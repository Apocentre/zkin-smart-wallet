use anchor_lang::prelude::*;
use groth16_solana::groth16::{self, Groth16Verifier};
use crate::{
  account_data::zkp::Zkp, program_error::ErrorCode, zk::verifying_key::VERIFYING_KEY
};

pub fn prepare_input(zkp: &mut Zkp) -> Result<()> {
  let prepared_public_inputs = groth16::prepare_inputs(
    &VERIFYING_KEY,
    zkp.prepared_public_inputs,
    zkp.convert_public_inputs()?,
    zkp.offset(),
  ).map_err(|_| ErrorCode::InvalidProofData)?;

  zkp.prepared_public_inputs = Some(prepared_public_inputs);
  zkp.next_iteration();

  Ok(())
}


pub fn verify_proof(zkp: &Zkp, owner: Pubkey) -> Result<()> {
  // The signer of the tx must be the nonce claim value from JWT.
  // This way we are sure that the transactionw as signed by the user who pocess a valid JWT
  require!(zkp.nonce()?.eq(&owner), ErrorCode::InvalidAccount);

  // TODO: check expiration time
  // TODO: make sure the the rsa_modulo belongs to the iss

  let mut verifier = Groth16Verifier::new(
    &zkp.proof_a,
    &zkp.proof_b,
    &zkp.proof_c,
    zkp.prepared_public_inputs.expect("prepare inputs"),
    &VERIFYING_KEY,
  ).map_err(|_| ErrorCode::InvalidProofData)?;
  
  verifier.verify().map_err(|_| ErrorCode::GrothVerificationError)?;

  Ok(())
}
