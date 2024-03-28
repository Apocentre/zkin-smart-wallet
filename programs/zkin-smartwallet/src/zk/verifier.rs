use anchor_lang::prelude::*;
use groth16_solana::groth16::{self, Groth16Verifier};
use crate::{
  account_data::{auth_provider::AuthProvider, zkp::Zkp}, program_error::ErrorCode, zk::verifying_key::VERIFYING_KEY
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


pub fn verify_proof(
  zkp: &Zkp,
  owner: Pubkey,
  auth_provider: &AuthProvider,
  provider: String,
  now: i64,
) -> Result<()> {
  // The signer of the tx must be the nonce claim value from JWT.
  // This way we are sure that the transactionw as signed by the user who pocess a valid JWT
  require!(zkp.nonce()?.eq(&owner), ErrorCode::InvalidAccount);
  // make sure the the iss from the ZKP is the same with the one user provided. The user provided value is used
  // as seed in the AuthProvider PDA derivation so we need to check the correct PDA is provided.
  require!(zkp.iss()? == provider, ErrorCode::WrongAuthProviderProvided);
  // Check that RSA modulus from the ZKP is registered by the operator. This way we verify that
  // the correct (and one that we support) auth provider (iss) signed the JWT
  require!(auth_provider.modulus_registered(zkp.rsa_modulus()?), ErrorCode::InvalidRsaKey);
  // check that the JWT token is not expired
  require!(now >= zkp.exp()?, ErrorCode::TokenExpired);

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
