use std::io::Write;
use anchor_lang::prelude::*;

pub const PUBLIC_INPUTS_LEN: usize = 308;

fn pad_value(mut bytes: &mut [u8], s: &str) {
  bytes.write(s.as_bytes()).unwrap();
}

#[account]
pub struct Zkp {
  /// The proof_a of the ZKP
  pub proof_a: [u8; 64],
  /// The proof_b of the ZKP
  pub proof_b: [u8; 128],
  // The proof_c of the ZKP
  pub proof_c: [u8; 64],
  /// The public inputs of the circuit. We use fixed size because we know what the
  /// len of the public inputs are for the ZkIn circuit.
  /// note we have 246 inputs. The first 244 are in u8 which will then be converted to [u8; 32] to work with
  /// the Circom Field. The last two are already in [u8; 32] i.e. the address Poseidon hash and the rsa_modulo.
  /// We will split them into [u8; 32] in the processor
  pub public_inputs: [u8; PUBLIC_INPUTS_LEN],
  /// The intermediate value of the prepared_public_inputs
  pub prepared_public_inputs: Option<[u8; 64]>,
  /// The current iteration starting from 0 and incremented after each public input batch is processed
  pub iteration: u8,
  /// The size of the origian public inputs that will be processed during each iteration
  pub batch_size: u8,
  /// The zkp PDA bump
  pub bump: u8,
}

impl Zkp {
  pub fn new(
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    public_inputs: [u8; PUBLIC_INPUTS_LEN],
    batch_size: u8,
    bump: u8,
  ) -> Self {
    Self {
      proof_a,
      proof_b,
      proof_c,
      prepared_public_inputs: None,
      public_inputs,
      batch_size,
      iteration: 0,
      bump,
    }
  }

  /// Convert public inputs slice to array of [u8; 32]. Each u8 becomes [u8; 32]
  /// It will convert only the section of the public inputs that is relevant for the current iteration.
  pub fn convert_public_inputs(&self) -> Vec<[u8; 32]> {
    let mut result = Vec::new();
    let start = self.offset();
    let end = start + self.batch_size as usize;

    let mut iterate = |start: usize, end: usize| {
      for val in self.public_inputs[start..end].iter() {
        let mut bytes = [0; 32];
        let item = hex::encode([*val]);
        pad_value(&mut bytes, &item);
    
        result.push(bytes);
      }
    };

    if end > 244 {
      iterate(start, 244);

      // address is already a 32 bytes hex value
      result.push(self.public_inputs[244..276].try_into().unwrap());
      // so is rsa_modulo
      result.push(self.public_inputs[276..308].try_into().unwrap());
    } else {
      iterate(start, end);
    }

    result
  }

  pub fn offset(&self) -> usize {
    (self.batch_size * self.iteration) as usize
  }

  pub fn next_iteration(&mut self) {
    self.iteration += 1;
  }
}
