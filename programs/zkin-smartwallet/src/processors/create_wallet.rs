use anchor_lang::prelude::*;
use crate::instructions::create_wallet::CreateWallet;

pub fn exec(ctx: Context<CreateWallet>, wallet_address: [u8; 32]) -> Result<()> {
  let wallet_address = hex::encode(wallet_address);

  // verify_proof(
  //   proof_a,
  //   proof_b,
  //   proof_c,
  //   public_inputs_vec,
  // )?;

  // update wallet state
  let wallet = &mut ctx.accounts.wallet;
  wallet.address = wallet_address;
  wallet.owner = ctx.accounts.owner.key();
  wallet.bump = ctx.bumps.wallet;

  Ok(())
}
