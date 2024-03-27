use anchor_lang::prelude::*;
use crate::{
  account_data::{
    auth_provider::AuthProvider,
    state::State
  }, instruction, program_error::ErrorCode
};

#[derive(Accounts)]
#[instruction(provider: String)]
pub struct RegisterRsaModulus<'info> {
  /// The state account of each instance of this program
  #[account(mut)]
  pub state: Account<'info, State>,

  /// The state account of each instance of this program
  #[account(
    init_if_needed,
    payer = operator,
    space = 0,
    seeds = [state.key().as_ref(), provider.as_ref()],
    bump
  )]
  pub auth_provider: Account<'info, AuthProvider>,

  #[account(
    mut,
    constraint = state.operators.iter().any(|c| c.key() == operator.key()) @ ErrorCode::OnlyOperator,
  )]
  pub operator: Signer<'info>,

  pub system_program: Program<'info, System>,
}
