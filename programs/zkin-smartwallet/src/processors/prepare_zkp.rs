use anchor_lang::prelude::*;
use crate::{
  instructions::prepare_zkp::PrepareZkp, zk::verifier::prepare_input
};

pub fn exec(ctx: Context<PrepareZkp>) -> Result<()> {
  prepare_input(&mut ctx.accounts.zkp)?;
  Ok(())
}
