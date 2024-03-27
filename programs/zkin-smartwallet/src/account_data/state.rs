use anchor_lang::prelude::*;

pub const MAX_OPERATORS: usize = 5;

#[account]
pub struct State {
  /// The owner that can handle various admin related teasks, e.g update operators
  pub owner: Pubkey,

  /// The list of all operators that can run admin related tasks e.g. update auth rsa
  pub operators: [Pubkey; MAX_OPERATORS],
}

impl State {
  pub fn new(owner: Pubkey) -> Self {
    let mut operators = [Pubkey::default(); MAX_OPERATORS];
    operators[0] = owner;

    Self {
      owner,
      operators,
    }
  }
}
