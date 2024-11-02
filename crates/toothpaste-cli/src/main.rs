use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use std::io::{self, IsTerminal, Read};
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

    // Check if stdin is connected to a terminal
    if io::stdin().is_terminal() {
        return Err("No input provided. Please pipe or redirect input to the program.".into());
    }

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
    let api_url: &'static str = env!("TOOTHPASTE_API_URL");
    let api_route = format!("{}/api/paste/new", api_url);

    let resp = ureq::post(&api_route).send_json(&encrypted_paste)?;

    // Get paste ID from response
    let paste_response: PasteCreateResponse = resp.into_json()?;

    // Generate key in base64
    let key_base64 = general_purpose::URL_SAFE_NO_PAD.encode(key);

    // Print the URL
    println!("{}/paste/{}#{}", api_url, paste_response.id, key_base64);

    Ok(())
}
