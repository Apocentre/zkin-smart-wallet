use anchor_lang::prelude::*;
use crate::{
  instructions::create_wallet::CreateWallet, zk::verifier::verify_proof
};

pub fn exec(ctx: Context<CreateWallet>, wallet_address: [u8; 32]) -> Result<()> {
  let wallet_address = hex::encode(wallet_address);

  verify_proof(&ctx.accounts.zkp)?;

  // update wallet state
  let wallet = &mut ctx.accounts.wallet;
  wallet.address = wallet_address;
  wallet.owner = ctx.accounts.owner.key();
  wallet.bump = ctx.bumps.wallet;

  Ok(())
}
