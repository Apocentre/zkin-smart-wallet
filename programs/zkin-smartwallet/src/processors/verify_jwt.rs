use anchor_lang::prelude::*;
use rsa::{RSAPublicKey, BigUint, PublicKey, Hash};
use sha2::{Sha256, Digest};
use crate::{
  instructions::verify_jwt::VerifyJwt,
  program_error::ErrorCode,
};

const GOOGLE_PUBKEY_N: &[u8] = &[171, 10, 243, 151, 78, 159, 192, 30, 142, 34, 110, 182, 35, 19, 70, 236, 213, 205, 32, 57, 160, 116, 26, 239, 127, 79, 104, 111, 97, 172, 167, 166, 102, 1, 120, 20, 226, 218, 199, 61, 70, 43, 116, 22, 128, 149, 78, 90, 30, 151, 33, 92, 39, 255, 153, 82, 254, 83, 151, 115, 70, 167, 149, 21, 11, 21, 170, 200, 248, 60, 66, 4, 232, 78, 78, 63, 15, 4, 194, 233, 143, 217, 252, 203, 65, 43, 48, 166, 66, 166, 36, 146, 133, 113, 217, 220, 125, 14, 247, 209, 0, 104, 157, 21, 4, 132, 169, 8, 110, 38, 29, 137, 194, 101, 231, 36, 228, 57, 23, 116, 252, 25, 159, 116, 181, 125, 242, 191, 172, 19, 1, 7, 55, 137, 115, 76, 157, 121, 251, 44, 61, 210, 134, 255, 71, 171, 192, 31, 140, 15, 200, 36, 141, 221, 119, 185, 255, 53, 122, 71, 129, 13, 47, 99, 101, 42, 125, 135, 79, 217, 91, 87, 177, 225, 181, 10, 39, 129, 230, 188, 193, 249, 237, 133, 72, 93, 100, 125, 252, 144, 142, 18, 75, 153, 192, 104, 37, 77, 196, 64, 51, 150, 242, 109, 80, 135, 66, 165, 40, 35, 82, 7, 27, 106, 31, 131, 193, 141, 253, 151, 128, 252, 211, 73, 11, 198, 71, 5, 216, 33, 151, 61, 41, 169, 23, 193, 143, 179, 86, 150, 74, 198, 46, 165, 143, 44, 77, 44, 189, 168, 33, 73, 20, 241, 113, 241];
const GOOGLE_PUBKEY_E: &[u8] = &[1, 0, 1];

pub fn exec(
  _ctx: Context<VerifyJwt>,
  header: Vec<u8>,
  payload: Vec<u8>,
  sig: Vec<u8>,
) -> Result<()> {
  // We need to sha256 the following message <header>.<payload>
  let mut msg = header;
  msg.extend(b".");
  msg.extend(payload);
  
  let mut hasher = Sha256::new();
  hasher.update(&msg);
  let msg_hash = hasher.finalize();

  let google_pubkey = RSAPublicKey::new(
    BigUint::from_bytes_le(GOOGLE_PUBKEY_N),
    BigUint::from_bytes_le(GOOGLE_PUBKEY_E),
  ).map_err(|_| ErrorCode::RsaError)?;

  google_pubkey.verify(
    rsa::PaddingScheme::PKCS1v15Sign {hash: Some(Hash::SHA2_256)},
    &msg_hash,
    &sig
  ).map_err(|_| ErrorCode::RsaError)?;

  Ok(())
}
