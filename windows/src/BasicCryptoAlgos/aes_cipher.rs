use ring::aead;

pub fn encryptAes(plaintext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // AES-GCM requires a 256-bit (32-byte) key
    let key_bytes = b"";
    let key = aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes).unwrap();
    let sealing_key = aead::LessSafeKey::new(key);

    // Fixed IV (12 bytes for AES-GCM)
    let iv = b""; // Ensure this is exactly 12 bytes
    let nonce = aead::Nonce::assume_unique_for_key(*iv);

    // Allocate space for ciphertext: plaintext length + tag length
    let mut in_out = plaintext.to_vec();
    in_out.resize(plaintext.len() + aead::AES_256_GCM.tag_len(), 0);

    // Encrypt the plaintext
    sealing_key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out).unwrap();

    Ok(in_out)
}