use anchor_lang::prelude::*;
use crate::{
  instructions::create_wallet::CreateWallet, zk::verifier::verify_proof,
};

pub fn exec(
  ctx: Context<CreateWallet>,
  wallet_address: String,
  proof_a: [u8; 64],
  proof_b: [u8; 128],
  proof_c: [u8; 64],
  public_inputs_vec: Vec<u8>,
) -> Result<()> {
  verify_proof(
    wallet_address,
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
