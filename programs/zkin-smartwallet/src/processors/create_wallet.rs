use anchor_lang::prelude::*;
use crate::{
  instructions::create_wallet::CreateWallet, zk::verifier::verify_proof,
  program_error::ErrorCode,
};

pub fn exec(
  ctx: Context<CreateWallet>,
  wallet_address: [u8; 32],
  provider: String,
  now: i64,
) -> Result<()> {
  let zkp = &ctx.accounts.zkp;
  require!(zkp.address()? == wallet_address, ErrorCode::WrongWalletProvided);

  verify_proof(
    zkp,
    ctx.accounts.owner.key(),
    &ctx.accounts.auth_provider,
    provider,
    now,
  )?;
  
  // update wallet state
  let wallet = &mut ctx.accounts.wallet;
  wallet.address = bs58::encode(zkp.address()?).into_string();
  wallet.owner = ctx.accounts.owner.key();
  wallet.bump = ctx.bumps.wallet;

  Ok(())
}
