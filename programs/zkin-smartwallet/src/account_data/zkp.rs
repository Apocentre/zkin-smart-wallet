use std::mem::size_of;
use anchor_lang::prelude::*;

#[account]
pub struct Zkp {
  // The proof_a of the ZKP
  proof_a: [u8; 64],
  // The proof_b of the ZKP
  proof_b: [u8; 128],
  // The proof_c of the ZKP
  proof_c: [u8; 64],
  // The public inputs of the circuit. We use fixed size because we know what the
  // len of the public inputs are for the ZkIn circuit.
  // note we have 246 inputs. The first 244 are in u8 which will then be converted to [u8; 32] to work with
  // the Circom Field. The last two are already in [u8; 32] i.e. the address Poseidon hash and the rsa_modulo.
  // We will split them into [u8; 32] in the processor
  public_inputs: [u8; 308],
}
