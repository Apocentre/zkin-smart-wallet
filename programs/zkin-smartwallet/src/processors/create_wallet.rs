use std::str;
use anchor_lang::prelude::*;
use crate::{
  instructions::create_wallet::CreateWallet, program_error::ErrorCode,
  // zk::verifier::verify_proof,
};

pub fn exec(
  ctx: Context<CreateWallet>,
  proof_a: [u8; 64],
  proof_b: [u8; 128],
  proof_c: [u8; 64],
  public_inputs_vec: [u8; 308],
) -> Result<()> {
  msg!("1 >>>>>>>>>>>>>>>");
  let wallet_address = &public_inputs_vec[244..276];
  msg!("2 >>>>>>>>>>>>>>> {:?}", wallet_address);
  let wallet_address = str::from_utf8(&wallet_address).map_err(|_| ErrorCode::InvalidWalletAddress)?;
  msg!("3 >>>>>>>>>>>>>>> {wallet_address:?}");


  // verify_proof(
  //   proof_a,
  //   proof_b,
  //   proof_c,
  //   public_inputs_vec,
  // )?;

  // update wallet state
  let wallet = &mut ctx.accounts.wallet;
  wallet.address = wallet_address.to_owned();
  wallet.owner = ctx.accounts.owner.key();
  wallet.bump = ctx.bumps.wallet;

  Ok(())
}
