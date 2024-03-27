use anchor_lang::prelude::*;
use crate::account_data::{
  wallet::{wallet_size, Wallet}, zkp::Zkp,
};

#[derive(Accounts)]
#[instruction(wallet_address: [u8; 32])]
pub struct CreateWallet<'info> {
  /// CHECK: The PDA that represent the actial wallet
  #[account(
    init,
    payer = operator,
    space = wallet_size(),
    seeds = [wallet_address.as_ref()],
    bump,
  )]
  pub wallet: Account<'info, Wallet>,
  
  /// CHECK: The PDA that represent the ZKP data
  #[account(
    seeds = [b"zkp", wallet_address.as_ref()],
    bump = zkp.bump,
  )]
  pub zkp: Account<'info, Zkp>,
  
  /// The operator that partially signs the tx and pays for the gas
  #[account(mut)]
  pub operator: Signer<'info>,

  /// The actual owner of the wallet that partially signs the tx but does not pay the gas fees
  /// This must be the same as the `nonce` claim in the Zkp
  #[account(mut)]
  pub owner: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
