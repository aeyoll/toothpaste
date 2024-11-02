use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use std::io::{self, Read};
use toothpaste_encrypt::{
    encrypt, generate_key, generate_nonce, EncryptedPaste, PasteCreateResponse,
};

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
