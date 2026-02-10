use aes::Aes128;
use cipher::{BlockDecrypt, KeyInit};
use cipher::generic_array::GenericArray;

pub fn aes_ecb_128_decode(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if key.len() != 16 {
        return Err("Key must be 16 bytes".into());
    }
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut plaintext = Vec::new();
    for chunk in ciphertext.chunks_exact(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        plaintext.extend_from_slice(&block);
    }
    if let Some(&pad_len) = plaintext.last() {
        let pad_len = pad_len as usize;
        if pad_len <= 16 {
            plaintext.truncate(plaintext.len() - pad_len);
        }
    }
    Ok(plaintext)
}