use anchor_lang::prelude::*;
use crate::{
  account_data::state::MAX_OPERATORS, instructions::update_operators::UpdateOperators,
};

pub fn exec(ctx: Context<UpdateOperators>, operators: [Pubkey; MAX_OPERATORS]) -> Result<()> {
  let state = &mut ctx.accounts.state;
  state.operators = operators;

  Ok(())
}
