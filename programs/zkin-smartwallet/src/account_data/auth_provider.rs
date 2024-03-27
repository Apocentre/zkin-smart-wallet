use anchor_lang::prelude::*;

// Each Auth provider rotates the RSA keys that it uses to sign JWT.
// For example, google https://www.googleapis.com/oauth2/v3/certs has 3 active ones
// at a time. We can use the 
pub const MAX_RSA_MODULO: usize = 10;

#[account]
pub struct AuthProvider {
  /// A cyclic array storing the latest auth provider RSA keys. More specifically this is the modulus value
  /// which is the value of the field `n` returned by the JWKS endpoint of the auth provider`
  pub rsa_modulus: [[u8; 32]; MAX_RSA_MODULO],
  /// The bump of the PDA account
  pub bump: u8,
}

impl AuthProvider {
  pub fn new(bump: u8) -> Self {
    let rsa_modulus = [[0; 32]; MAX_RSA_MODULO];

    Self {
      rsa_modulus,
      bump,
    }
  }
}
