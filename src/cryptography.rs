use aes_gcm_siv::aead::{Aead, OsRng};
use aes_gcm_siv::{AeadCore, Aes256GcmSiv, KeyInit, Nonce};

pub type Key = [u8; 32];

// Generate a random key
pub fn random_key() -> Key {
    Aes256GcmSiv::generate_key(&mut OsRng).into()
}

/// Generate a random nonce for AES-GCM.
/// AES-GCM nonces are 12 bytes (96 bits)
pub fn random_nonce() -> Nonce {
    Aes256GcmSiv::generate_nonce(&mut OsRng).into()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cryptography {
    key: Key,
    nonce: Nonce,
}

impl Cryptography {
    pub fn new() -> Self {
        Self {
            key: random_key(),
            nonce: random_nonce(),
        }
    }

    pub fn init(key: Key, nonce: Nonce) -> Self {
        Self { key, nonce }
    }

    pub fn key(&self) -> Key {
        self.key
    }

    pub fn nonce(&self) -> Nonce {
        self.nonce
    }

    pub fn encrypt(&self, plain_text: String) -> Vec<u8> {
        let cipher = Aes256GcmSiv::new(&self.key.into());
        let cipher_text = cipher.encrypt(&self.nonce, plain_text.as_bytes()).unwrap();

        cipher_text
    }

    pub fn decrypt(&self, cipher_text: Vec<u8>) -> Vec<u8> {
        let cipher = Aes256GcmSiv::new(&self.key.into());
        let plain_text = cipher.decrypt(&self.nonce, cipher_text.as_ref()).unwrap();

        plain_text
    }
}
