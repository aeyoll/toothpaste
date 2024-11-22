use aes_gcm::aead::consts::U12;
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, Nonce, OsRng};
use aes_gcm::aes::Aes256;
use aes_gcm::{AeadCore, Aes256Gcm, AesGcm, KeyInit};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PasteCreateResponse {
    pub id: String,
}

#[derive(Serialize)]
pub struct EncryptedPaste {
    pub filename: String,
    pub content: String,
    pub expire_after: i64,
}

#[derive(Deserialize, Debug)]
pub struct PasteResponse {
    pub filename: String,
    pub content: String,
    pub expire_time: Option<String>,
}

pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

pub fn generate_nonce() -> Nonce<AesGcm<Aes256, U12>> {
    Aes256Gcm::generate_nonce(&mut OsRng)
}

pub fn encrypt(
    data: &str,
    nonce: &Nonce<AesGcm<Aes256, U12>>,
    key: &[u8; 32],
) -> Result<String, String> {
    let cipher = Aes256Gcm::new(key.into());
    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| format!("Encryption error: {:?}", e))?;

    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(general_purpose::URL_SAFE_NO_PAD.encode(result))
}

pub fn decrypt_paste(paste: &PasteResponse, key_base64: &str) -> Result<PasteResponse, String> {
    // Decode the base64 key
    let key = general_purpose::URL_SAFE_NO_PAD
        .decode(key_base64)
        .map_err(|e| format!("Failed to decode key: {:?}", e))?;

    if key.len() != 32 {
        return Err("Invalid key length".to_string());
    }

    let key: [u8; 32] = key.try_into().unwrap();

    // Create cipher instance
    let cipher = Aes256Gcm::new(&key.into());

    // Decrypt filename
    let filename = decrypt_data(&cipher, &paste.filename)?;

    // Decrypt content
    let content = decrypt_data(&cipher, &paste.content)?;

    Ok(PasteResponse {
        filename,
        content,
        expire_time: paste.expire_time.clone(),
    })
}

fn decrypt_data(cipher: &Aes256Gcm, data: &str) -> Result<String, String> {
    let decoded = general_purpose::URL_SAFE_NO_PAD
        .decode(data)
        .map_err(|e| format!("Failed to decode data: {:?}", e))?;

    if decoded.len() < 12 {
        return Err("Invalid data length".to_string());
    }

    let nonce = Nonce::<AesGcm<Aes256, U12>>::from_slice(&decoded[0..12]);
    let ciphertext = &decoded[12..];

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption error: {:?}", e))?;

    String::from_utf8(plaintext).map_err(|e| format!("UTF-8 decoding error: {:?}", e))
}
