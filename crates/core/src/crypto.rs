use crate::error::Result;
use hkdf::Hkdf;
use sha2::Sha256;
use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, Payload}};
use zeroize::Zeroize;

pub struct Crypto;

impl Crypto {
    pub fn derive_key(password: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
        let hk = Hkdf::<Sha256>::new(Some(salt), password);
        let mut okm = [0u8; 32];
        hk.expand(b"librepods-v1.0.0", &mut okm)
            .map_err(|_| crate::error::Error::CryptoError)?;
        Ok(okm.to_vec())
    }

    pub fn hash(data: &[u8]) -> Vec<u8> {
        blake3::hash(data).as_bytes().to_vec()
    }

    pub fn verify_hash(data: &[u8], hash: &[u8]) -> bool {
        let calculated = Self::hash(data);
        constant_time_eq(&calculated, hash)
    }

    pub fn encrypt(key: &[u8], plaintext: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(crate::error::Error::CryptoError);
        }
        if nonce.len() != 12 {
            return Err(crate::error::Error::CryptoError);
        }
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|_| crate::error::Error::CryptoError)?;
        let nonce_arr = aes_gcm::Nonce::from_slice(nonce);
        cipher.encrypt(nonce_arr, Payload::from(plaintext))
            .map_err(|_| crate::error::Error::CryptoError)
    }

    pub fn decrypt(key: &[u8], ciphertext: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(crate::error::Error::CryptoError);
        }
        if nonce.len() != 12 {
            return Err(crate::error::Error::CryptoError);
        }
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|_| crate::error::Error::CryptoError)?;
        let nonce_arr = aes_gcm::Nonce::from_slice(nonce);
        cipher.decrypt(nonce_arr, Payload::from(ciphertext))
            .map_err(|_| crate::error::Error::CryptoError)
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
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
