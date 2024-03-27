use anchor_lang::prelude::*;
use crate::account_data::{
  auth_provider::AuthProvider, wallet::Wallet, zkp::Zkp
};

#[derive(Accounts)]
#[instruction(wallet_address: [u8; 32], provider: String)]
pub struct CreateWallet<'info> {
    /// CHECK: The state account of each instance of this program, we don't need to do any checks
    #[account()]
    pub state: AccountInfo<'info>,
  
    /// The auth provider pda
    #[account(
      seeds = [state.key().as_ref(), provider.as_ref()],
      bump
    )]
    pub auth_provider: Account<'info, AuthProvider>,

  /// CHECK: The PDA that represent the actial wallet
  #[account(
    init,
    payer = operator,
    space = Wallet::size(),
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
