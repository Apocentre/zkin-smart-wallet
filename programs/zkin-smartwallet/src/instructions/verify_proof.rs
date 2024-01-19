use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VerifyProof<'info> {
  #[account(mut)]
  user: Signer<'info>,
}
