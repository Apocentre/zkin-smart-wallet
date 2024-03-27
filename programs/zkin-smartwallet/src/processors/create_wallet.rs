use anchor_lang::prelude::*;
use crate::{
  instructions::create_wallet::CreateWallet, zk::verifier::verify_proof,
  program_error::ErrorCode,
};

pub fn exec(ctx: Context<CreateWallet>, wallet_address: [u8; 32]) -> Result<()> {
  let zkp = &ctx.accounts.zkp;
  require!(zkp.address()? == wallet_address, ErrorCode::WrongWalletProvided);
  
  verify_proof(zkp, ctx.accounts.owner.key())?;
  
  // update wallet state
  let wallet = &mut ctx.accounts.wallet;
  wallet.address = String::from_utf8(zkp.address()?.to_vec()).unwrap();
  wallet.owner = ctx.accounts.owner.key();
  wallet.bump = ctx.bumps.wallet;

  Ok(())
}
