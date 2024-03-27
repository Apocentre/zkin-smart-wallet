use anchor_lang::prelude::*;
use crate::{
  instructions::create_wallet::CreateWallet, zk::verifier::verify_proof
};

pub fn exec(ctx: Context<CreateWallet>, wallet_address: [u8; 32]) -> Result<()> {
  let wallet_address = hex::encode(wallet_address);

  // TODO: make sure the rsa_modulo is registered under the iss claim. We need to have a state account
  // that store the rsa pubkeys of the auth providers (iss) we support.
  // Basically, require!(state.belogs_to_provider())

  verify_proof(&ctx.accounts.zkp)?;


  // update wallet state
  let wallet = &mut ctx.accounts.wallet;
  wallet.address = wallet_address;
  wallet.owner = ctx.accounts.owner.key();
  wallet.bump = ctx.bumps.wallet;

  Ok(())
}
