use aes_gcm::aead::consts::U12;
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, Nonce, OsRng};
use aes_gcm::aes::Aes256;
use aes_gcm::{AeadCore, Aes256Gcm, AesGcm, KeyInit};
use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Filename for the paste
    #[arg(short, long, default_value = "toothpaste.txt")]
    filename: String,

    /// Expiration time in seconds (default: 86400 - 1 day)
    #[arg(short, long, default_value = "86400")]
    expire_after: i64,
}

#[derive(Deserialize)]
struct PasteCreateResponse {
    id: String,
}

#[derive(Serialize)]
struct EncryptedPaste {
    filename: String,
    content: String,
    expire_after: i64,
}

fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

fn generate_nonce() -> Nonce<AesGcm<Aes256, U12>> {
    Aes256Gcm::generate_nonce(&mut OsRng)
}

fn encrypt(
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Read input from stdin
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;

    if content.is_empty() {
        return Err("Empty content".into());
    }

    // Generate encryption key and nonce
    let key = generate_key();
    let nonce = generate_nonce();

    // Encrypt content and filename
    let encrypted_filename = encrypt(&cli.filename, &nonce, &key)?;
    let encrypted_content = encrypt(&content, &nonce, &key)?;

    // Create encrypted paste object
    let encrypted_paste = EncryptedPaste {
        filename: encrypted_filename,
        content: encrypted_content,
        expire_after: cli.expire_after,
    };

    // Send request to API
    let api_url =
        std::env::var("TOOTHPASTE_API_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

    let resp = ureq::post(&format!("{}/api/paste/new", api_url)).send_json(&encrypted_paste)?;

    // Get paste ID from response
    let paste_response: PasteCreateResponse = resp.into_json()?;

    // Generate key in base64
    let key_base64 = general_purpose::URL_SAFE_NO_PAD.encode(key);

    // Print the URL
    println!("{}/paste/{}#{}", api_url, paste_response.id, key_base64);

    Ok(())
}
