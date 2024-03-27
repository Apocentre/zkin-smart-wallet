use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Wrong wallet provider")]
  WrongWalletProvided,
  #[msg("Invalid Groth16 proof data")]
  InvalidProofData,
  #[msg("Invalid account")]
  InvalidAccount,
  #[msg("Groth verification error")]
  GrothVerificationError,
  #[msg("Corrupted public signals")]
  CorruptedPublicInputs,
  #[msg("Only operator")]
  OnlyOperator,
}
