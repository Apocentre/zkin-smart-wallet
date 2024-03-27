use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::account_data::state::State;

#[derive(Accounts)]
pub struct Initialize<'info> {
  /// The state account of each instance of this program
  #[account(
    init,
    payer = owner,
    space = 8 + size_of::<State>()
  )]
  pub state: Account<'info, State>,
  
  #[account(mut)]
  pub owner: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
