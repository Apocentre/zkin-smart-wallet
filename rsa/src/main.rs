use rsa::{RSAPublicKey, BigUint};
use base64::{decode_config, URL_SAFE_NO_PAD};

const GOOGLE_PUBKEY_N: &str = "qwrzl06fwB6OIm62IxNG7NXNIDmgdBrvf09ob2Gsp6ZmAXgU4trHPUYrdBaAlU5aHpchXCf_mVL-U5dzRqeVFQsVqsj4PEIE6E5OPw8EwumP2fzLQSswpkKmJJKFcdncfQ730QBonRUEhKkIbiYdicJl5yTkORd0_BmfdLV98r-sEwEHN4lzTJ15-yw90ob_R6vAH4wPyCSN3Xe5_zV6R4ENL2NlKn2HT9lbV7HhtQongea8wfnthUhdZH38kI4SS5nAaCVNxEAzlvJtUIdCpSgjUgcbah-DwY39l4D800kLxkcF2CGXPSmpF8GPs1aWSsYupY8sTSy9qCFJFPFx8Q";
const GOOGLE_PUBKEY_E: &str = "AQAB";


fn main() {
  let n = decode_config(&GOOGLE_PUBKEY_N, URL_SAFE_NO_PAD).unwrap();
  let e = decode_config(&GOOGLE_PUBKEY_E, URL_SAFE_NO_PAD).unwrap();
  
  println!(">>>>>>>>> {:?}", n);
  println!(">>>>>>>>> {:?}", e);

  let google_pubkey = RSAPublicKey::new(
    BigUint::from_bytes_le(&n),
    BigUint::from_bytes_le(&e),
  ).unwrap();
}
