use anchor_lang::prelude::*;
use crate::{
  instructions::create_wallet::CreateWallet, zk::verifier::verify_proof,
  program_error::ErrorCode,
};

pub fn exec(ctx: Context<CreateWallet>, wallet_address: [u8; 32]) -> Result<()> {
  let wallet_address = hex::encode(wallet_address);

  // TODO: make sure the rsa_modulo is registered under the iss claim. We need to have a state account
  // that store the rsa pubkeys of the auth providers (iss) we support.
  // Basically, require!(state.belongs_to_provider(&ctx.accounts.zkp))

  let zkp = &ctx.accounts.zkp;
  verify_proof(zkp)?;

  // The signer of the tx must be the nonce claim value from JWT.
  // This way we are sure that the transactionw as signed by the user who pocess a valid JWT
  require!(zkp.nonce()?.eq(&ctx.accounts.owner.key()), ErrorCode::InvalidAccount);

  // update wallet state
  let wallet = &mut ctx.accounts.wallet;
  wallet.address = wallet_address;
  wallet.owner = ctx.accounts.owner.key();
  wallet.bump = ctx.bumps.wallet;

  Ok(())
}
