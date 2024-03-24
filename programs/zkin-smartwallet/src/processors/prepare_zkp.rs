use anchor_lang::prelude::*;
use crate::{
  instructions::prepare_zkp::PrepareZkp
};

pub fn exec(
  ctx: Context<PrepareZkp>,
) -> Result<()> {
  Ok(())
}
