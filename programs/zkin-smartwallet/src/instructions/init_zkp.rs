use std::mem::size_of;
use anchor_lang::prelude::*;
use crate::account_data::zkp::Zkp;

#[derive(Accounts)]
#[instruction(wallet_address: [u8; 32])]
pub struct InitZkp<'info> {
  /// CHECK: The PDA that represent the actial wallet
  #[account(
    init_if_needed,
    payer = owner,
    space = size_of::<Zkp>(),
    seeds = [b"zkp", wallet_address.as_ref()],
    bump,
  )]
  pub zkp: Account<'info, Zkp>,
  
  #[account(mut)]
  pub owner: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
