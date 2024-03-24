use anchor_lang::prelude::*;
use crate::account_data::zkp::Zkp;

#[derive(Accounts)]
#[instruction(wallet_address: [u8; 32])]
pub struct PrepareZkp<'info> {
  /// CHECK: The PDA that represent the ZKP data
  #[account(
    seeds = [b"zkp", wallet_address.as_ref()],
    bump = zkp.bump,
  )]
  pub zkp: Account<'info, Zkp>,
}
