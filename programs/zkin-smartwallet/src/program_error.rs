use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Invalid Groth16 proof data")]
  InvalidProofData,
  #[msg("Groth verification error")]
  GrothVerificationError,
  #[msg("Corrupted public signals")]
  CorruptedPublicInputs,
}
