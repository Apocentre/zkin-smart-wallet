use anchor_lang::prelude::*;
use crate::{account_data::zkp::Zkp, instructions::init_zkp::InitZkp};

pub fn exec(
  ctx: Context<InitZkp>,
  proof_a: [u8; 64],
  proof_b: [u8; 128],
  proof_c: [u8; 64],
  public_inputs: [u8; 308],
) -> Result<()> {
  let wallet_address = &public_inputs[244..276];
  let wallet_address = hex::encode(wallet_address);

  // update wallet state
  let zkp = &mut ctx.accounts.zkp;
  **zkp = Zkp::new(proof_a, proof_b, proof_c, public_inputs, ctx.bumps.zkp);

  Ok(())
}
