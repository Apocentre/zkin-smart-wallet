use anchor_lang::prelude::*;
use crate::{
  account_data::auth_provider::AuthProvider, instructions::register_rsa_modulus::RegisterRsaModulus,
};

pub fn exec(ctx: Context<RegisterRsaModulus>, rsa_modulus: [u8; 32]) -> Result<()> {
  let auth_provider = &mut ctx.accounts.auth_provider;
  **auth_provider = AuthProvider::new(ctx.bumps.auth_provider);
  auth_provider.register_modulus(rsa_modulus);

  Ok(())
}
