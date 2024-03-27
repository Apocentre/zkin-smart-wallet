use anchor_lang::prelude::*;

// Each Auth provider rotates the RSA keys that it uses to sign JWT.
// For example, google https://www.googleapis.com/oauth2/v3/certs has 3 active ones
// at a time. We can use the 
pub const MAX_RSA_MODULO: usize = 10;

#[account]
pub struct AuthProvider {
  /// A cyclic array storing the latest auth provider RSA keys
  pub rsa_modulos: [[u8; 32]; MAX_RSA_MODULO],
}

impl AuthProvider {
  pub fn new() -> Self {
    let rsa_modulos = [[0; 32]; MAX_RSA_MODULO];

    Self {
      rsa_modulos,
    }
  }
}
