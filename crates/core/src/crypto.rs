use crate::error::Result;
use zeroize::Zeroize;

pub struct Crypto;

impl Crypto {
    pub fn derive_key(password: &[u8], _salt: &[u8]) -> Result<Vec<u8>> {
        let key = blake3::derive_key("librepods", password);
        Ok(key.to_vec())
    }

    pub fn hash(data: &[u8]) -> Vec<u8> {
        blake3::hash(data).as_bytes().to_vec()
    }

    pub fn verify_hash(data: &[u8], hash: &[u8]) -> bool {
        let calculated = Self::hash(data);
        calculated == hash
    }

    pub fn encrypt(key: &[u8], plaintext: &[u8], _nonce: &[u8]) -> Result<Vec<u8>> {
        let mut result = vec![0u8; plaintext.len()];
        for (i, byte) in plaintext.iter().enumerate() {
            result[i] = byte ^ key[i % key.len()];
        }
        Ok(result)
    }

    pub fn decrypt(key: &[u8], ciphertext: &[u8], _nonce: &[u8]) -> Result<Vec<u8>> {
        let mut result = vec![0u8; ciphertext.len()];
        for (i, byte) in ciphertext.iter().enumerate() {
            result[i] = byte ^ key[i % key.len()];
        }
        Ok(result)
    }
}

pub struct SecureKey {
    data: Vec<u8>,
}

impl SecureKey {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Drop for SecureKey {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}
