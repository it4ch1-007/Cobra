use core::str;
use orion::aead;

pub fn fnChaCha(ciphertext: &[u8]) -> Result<Vec<u8>, orion::errors::UnknownCryptoError>{
    let key = aead::SecretKey::from_slice(b"").unwrap();

    let mut decrypted_data = aead::open(&key, &ciphertext)?;
    Ok(decrypted_data)

}
