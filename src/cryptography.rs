// This module provides functionality for encryption and decryption using AES-GCM-SIV.
// AES-GCM-SIV is an authenticated encryption mode that provides confidentiality, integrity,
// and authenticity of data. It utilizes AES encryption in Galois Counter Mode (GCM) with
// a Synthetic Initialization Vector (SIV) for deterministic encryption.
// This implementation uses the aes_gcm_siv crate for AES-GCM-SIV operations.
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
    Aes256GcmSiv::generate_nonce(&mut OsRng)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cryptography {
    key: Key,
    nonce: Nonce,
}

impl Cryptography {
    /// Creates a new instance of Cryptography with a randomly generated key and nonce.
    pub fn new() -> Self {
        Self {
            key: random_key(),
            nonce: random_nonce(),
        }
    }

    /// Initializes Cryptography with a specified key and nonce.
    pub fn init(key: Key, nonce: Nonce) -> Self {
        Self { key, nonce }
    }

    /// Retrieves the key used by the Cryptography instance.
    pub fn key(&self) -> Key {
        self.key
    }

    /// Retrieves the nonce used by the Cryptography instance.
    pub fn nonce(&self) -> Nonce {
        self.nonce
    }

    /// Encrypts the provided plaintext using AES-GCM-SIV.
    pub fn encrypt(&self, plain_text: String) -> Vec<u8> {
        let cipher = Aes256GcmSiv::new(&self.key.into());
        let cipher_text = cipher.encrypt(&self.nonce, plain_text.as_bytes()).unwrap();

        cipher_text
    }

    /// Decrypts the provided ciphertext using AES-GCM-SIV.
    pub fn decrypt(&self, cipher_text: Vec<u8>) -> Vec<u8> {
        let cipher = Aes256GcmSiv::new(&self.key.into());
        let plain_text = cipher.decrypt(&self.nonce, cipher_text.as_ref()).unwrap();

        plain_text
    }
}
