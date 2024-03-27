use std::mem::size_of;
use anchor_lang::prelude::*;

// Each Auth provider rotates the RSA keys that it uses to sign JWT.
// For example, google https://www.googleapis.com/oauth2/v3/certs has 3 active ones
// at a time. We can use the 
pub const MAX_RSA_MODULO: usize = 10;

#[account]
pub struct AuthProvider {
  /// A cyclic array storing the latest auth provider RSA keys. More specifically this is the modulus value
  /// which is the value of the field `n` returned by the JWKS endpoint of the auth provider`
  pub rsa_modulus: Vec<[u8; 32]>,
  /// The bump of the PDA account
  pub bump: u8,
}

impl AuthProvider {
  pub fn new(bump: u8) -> Self {
    Self {
      rsa_modulus: vec![],
      bump,
    }
  }

  /// Registers a new modulus in a cyclic manner
  pub fn register_modulus(&mut self, rsa_modulus: [u8; 32]) {
    let count = self.rsa_modulus.len() + 1;

    if count >= MAX_RSA_MODULO {
      self.rsa_modulus[count % MAX_RSA_MODULO] = rsa_modulus;
    } else {
      self.rsa_modulus.push(rsa_modulus);
    }
  }

  /// Checks if the modulus is registed
  pub fn modulus_registered(&self, rsa_modulus: [u8; 32]) -> bool {
    self.rsa_modulus.iter().find(|r| **r == rsa_modulus).is_some()
  }

  pub fn size() -> usize {
    8 + size_of::<Self>() + (MAX_RSA_MODULO * size_of::<[u8; 32]>())
  }
}
