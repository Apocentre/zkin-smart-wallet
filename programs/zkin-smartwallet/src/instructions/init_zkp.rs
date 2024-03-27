use std::mem::size_of;
use anchor_lang::prelude::*;
use crate::account_data::zkp::Zkp;

#[derive(Accounts)]
#[instruction(wallet_address: [u8; 32])]
pub struct InitZkp<'info> {
  /// CHECK: The PDA that represent the ZKP data
  #[account(
    init_if_needed,
    payer = operator,
    space = Zkp::size(),
    seeds = [b"zkp", wallet_address.as_ref()],
    bump,
  )]
  pub zkp: Account<'info, Zkp>,
    
  /// The operatora that partially signs the tx and pays for the gas
  #[account(mut)]
  pub operator: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
