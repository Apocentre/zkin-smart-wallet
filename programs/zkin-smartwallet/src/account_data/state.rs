use anchor_lang::prelude::*;

pub const MAX_OPERATORS: usize = 5;

#[account]
pub struct State {
  /// The list of all operators that can run admin related tasks e.g. update auth rsa
  pub operators: [Pubkey; MAX_OPERATORS],
}
