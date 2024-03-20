use anchor_lang::prelude::*;
use crate::{
  instructions::create_wallet::CreateWallet, zk::verifier::verify_proof,
};

pub fn exec(
  ctx: Context<CreateWallet>,
  proof_a: [u8; 64],
  proof_b: [u8; 128],
  proof_c: [u8; 64],
  public_inputs_vec: [u8; 308],
) -> Result<()> {
  let wallet_address = &public_inputs_vec[244..276];
  let wallet_address = hex::encode(wallet_address);

  verify_proof(
    proof_a,
    proof_b,
    proof_c,
    public_inputs_vec,
  )?;

  // update wallet state
  let wallet = &mut ctx.accounts.wallet;
  wallet.address = wallet_address;
  wallet.owner = ctx.accounts.owner.key();
  wallet.bump = ctx.bumps.wallet;

  Ok(())
}
