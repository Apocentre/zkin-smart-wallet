use std::str::FromStr;
use anchor_lang::prelude::*;
use rsa::{RSAPublicKey, BigUint, PublicKey};
use sha2::{Sha256, Digest};
use crate::{
  instructions::verify_jwt::VerifyJwt,
  program_error::ErrorCode,
};

const GOOGLE_PUBKEY_N: &str = "qwrzl06fwB6OIm62IxNG7NXNIDmgdBrvf09ob2Gsp6ZmAXgU4trHPUYrdBaAlU5aHpchXCf_mVL-U5dzRqeVFQsVqsj4PEIE6E5OPw8EwumP2fzLQSswpkKmJJKFcdncfQ730QBonRUEhKkIbiYdicJl5yTkORd0_BmfdLV98r-sEwEHN4lzTJ15-yw90ob_R6vAH4wPyCSN3Xe5_zV6R4ENL2NlKn2HT9lbV7HhtQongea8wfnthUhdZH38kI4SS5nAaCVNxEAzlvJtUIdCpSgjUgcbah-DwY39l4D800kLxkcF2CGXPSmpF8GPs1aWSsYupY8sTSy9qCFJFPFx8Q";
const GOOGLE_PUBKEY_E: &str = "AQAB-oaAcBuKQvWc5E31kXm6d6vfaEZjrMc_KT3DsFdN0LcAkB-Q9oYcVl7YEgAN849ROKUs6onf7eukj1PHwDzIBgA9AExJaKen0wITvxQv3H_BRXB7m6hFkLbK5Jo18gl3UxJ7Em29peEwi8Psn7MuI7CwhFNchKhjZM9eaMX27tpDPqR15-I6CA5Zf94rabUGWYph5cFXKWPPr8dskQQ";

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
    BigUint::from_str(GOOGLE_PUBKEY_N).unwrap(),
    BigUint::from_str(GOOGLE_PUBKEY_E).unwrap(),
  ).map_err(|_| ErrorCode::RsaError)?;

  google_pubkey.verify(
    rsa::PaddingScheme::PKCS1v15Encrypt,
    &msg_hash,
    &sig
  ).map_err(|_| ErrorCode::RsaError)?;

  Ok(())
}
