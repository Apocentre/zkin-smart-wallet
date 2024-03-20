use anchor_lang::prelude::*;
use std::io::Write;
use groth16_solana::groth16::Groth16Verifier;
use crate::{
  zk::verifying_key::VERIFYING_KEY, program_error::ErrorCode,
};

fn pad_value(mut bytes: &mut [u8], s: &str) {
  bytes.write(s.as_bytes()).unwrap();
}

// Convert public inputs slice to array of [u8; 32]. Each u8 becomes [u8; 308]
fn convert_public_inputs(public_inputs: [u8; 308]) -> [[u8; 32]; 246] {
  let mut result = [[0; 32]; 246];

  for (i, val) in public_inputs[..244].iter().enumerate() {
    let mut bytes = [0; 32];
    let item = hex::encode([*val]);
    pad_value(&mut bytes, &item);

    result[i] = bytes;
  }

  // address is already a 32 bytes hex value
  result[244] = public_inputs[244..276].try_into().unwrap();
  // so is rsa_modulo
  result[245] = public_inputs[276..308].try_into().unwrap();

  result
}

pub fn verify_proof(
  proof_a: [u8; 64],
  proof_b: [u8; 128],
  proof_c: [u8; 64],
  public_inputs: [u8; 308],
) -> Result<()> {
  msg!("1 >>>>>>>>>>>>>>>>>>>>>>>>");
  let pub_inputs = convert_public_inputs(public_inputs);
  msg!("2 >>>>>>>>>>>>>>>>>>>>>>>>");

  let mut verifier = Groth16Verifier::new(
    &proof_a,
    &proof_b,
    &proof_c,
    &pub_inputs,
    &VERIFYING_KEY,
  ).map_err(|_| ErrorCode::InvalidProofData)?;
  
  verifier.verify().map_err(|_| ErrorCode::GrothVerificationError)?;

  Ok(())
}
