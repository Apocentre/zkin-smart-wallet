use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Wrong wallet provided")]
  WrongWalletProvided,
  #[msg("Wrong Auth provider provided")]
  WrongAuthProviderProvided,
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
  #[msg("Invalid rsa key")]
  InvalidRsaKey,
}
