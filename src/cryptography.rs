use aes_gcm_siv::aead::rand_core::RngCore;
use aes_gcm_siv::aead::{Aead, OsRng};
use aes_gcm_siv::{Aes256GcmSiv, KeyInit, Nonce};
use rand::thread_rng;

pub type Key = [u8; 32];

// Generate a random key
pub fn random_key() -> Key {
    Aes256GcmSiv::generate_key(&mut OsRng).into()
}

/// Generate a random nonce for AES-GCM.
/// AES-GCM nonces are 12 bytes (96 bits)
pub fn random_nonce() -> Nonce {
    let mut nonce = Nonce::default();
    thread_rng().fill_bytes(&mut nonce[0..12]);
    nonce
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

    pub fn decrypt(&self, key: Key, nonce: Nonce, cipher_text: String) -> Vec<u8> {
        let cipher = Aes256GcmSiv::new(&key.into());
        let plain_text = cipher.decrypt(&nonce, cipher_text.as_ref()).unwrap();

        plain_text
    }
}
