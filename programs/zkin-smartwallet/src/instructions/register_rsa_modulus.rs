use anchor_lang::prelude::*;
use crate::{
  account_data::{
    auth_provider::AuthProvider, state::State
  },
  program_error::ErrorCode,
};

#[derive(Accounts)]
#[instruction(provider: String)]
pub struct RegisterRsaModulus<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Account<'info, State>,

  /// The auth provider pda
  #[account(
    init_if_needed,
    payer = operator,
    space = AuthProvider::size(),
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
