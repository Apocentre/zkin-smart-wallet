use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Invalid Groth16 proof data")]
  InvalidProofData,
  #[msg("Groth verification rrror")]
  GrothVerificationError,
  #[msg("Invalid idToken")]
  InvalidIdToken,
  #[msg("RSA error")]
  RsaError,
}
