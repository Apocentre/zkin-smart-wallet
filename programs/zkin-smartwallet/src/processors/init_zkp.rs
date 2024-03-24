use anchor_lang::prelude::*;
use crate::{
  account_data::zkp::{Zkp, PUBLIC_INPUTS_LEN}, instructions::init_zkp::InitZkp
};

pub fn exec(
  ctx: Context<InitZkp>,
  proof_a: [u8; 64],
  proof_b: [u8; 128],
  proof_c: [u8; 64],
  public_inputs: [u8; PUBLIC_INPUTS_LEN],
  batch_size: u8,
) -> Result<()> {
  // update wallet state
  let zkp = &mut ctx.accounts.zkp;
  **zkp = Zkp::new(
    proof_a,
    proof_b,
    proof_c,
    public_inputs,
    batch_size,
    ctx.bumps.zkp,
  );

  Ok(())
}
