use std::io::Write;
use anchor_lang::prelude::*;
use groth16_solana::groth16::{self, Groth16Verifier};
use crate::{
  account_data::zkp::Zkp, program_error::ErrorCode, zk::verifying_key::VERIFYING_KEY
};

fn pad_value(mut bytes: &mut [u8], s: &str) {
  bytes.write(s.as_bytes()).unwrap();
}

// Convert public inputs slice to array of [u8; 32]. Each u8 becomes [u8; 32]
// fn convert_public_inputs(zkp: &Zkp) -> Vec<[u8; 32]> {
//   let mut result = Vec::new();

//   for (i, val) in public_inputs[..244].iter().enumerate() {
//     let mut bytes = [0; 32];
//     let item = hex::encode([*val]);
//     pad_value(&mut bytes, &item);

//     result[i] = bytes;
//   }

//   // address is already a 32 bytes hex value
//   result[244] = public_inputs[244..276].try_into().unwrap();
//   // so is rsa_modulo
//   result[245] = public_inputs[276..308].try_into().unwrap();

//   result
// }

// pub fn prepare_input(zkp: &mut Zkp) -> Result<()> {
//   groth16::prepare_inputs(
//     &VERIFYING_KEY,
//     zkp.prepared_public_inputs,
//     zkp.public_inputs,
//     zkp.offset,
//   );

//   Ok(())
// }


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
