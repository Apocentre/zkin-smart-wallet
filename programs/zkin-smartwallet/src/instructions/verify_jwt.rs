use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VerifyJwt<'info> {
  #[account(mut)]
  user: Signer<'info>,
}
