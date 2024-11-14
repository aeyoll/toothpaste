use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use std::io::{self, IsTerminal, Read};
use toothpaste_encrypt::{
    encrypt, generate_key, generate_nonce, EncryptedPaste, PasteCreateResponse,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// URL of the toothpaste API
    #[arg(
        short,
        long,
        env = "TOOTHPASTE_API_URL",
        default_value = "http://127.0.0.1:8000"
    )]
    url: String,

    /// name for the paste
    #[arg(short, long, default_value = "toothpaste.txt")]
    name: String,

    /// Expiration time in seconds (default: 86400 - 1 day)
    #[arg(short, long, default_value = "86400")]
    expire_after: i64,

    /// File to read content from
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut name = cli.name.clone();
    let mut content = String::new();

    if let Some(file) = cli.file {
        // Read content from file
        std::fs::File::open(&file)?.read_to_string(&mut content)?;

        // Set name to file name
        name = file.clone();
    } else {
        // Check if stdin is connected to a terminal
        if io::stdin().is_terminal() {
            return Err("No input provided. Please pipe or redirect input to the program.".into());
        }

        // Read content from stdin
        io::stdin().read_to_string(&mut content)?;
    }

    if content.is_empty() {
        return Err("Empty content".into());
    }

    // Generate encryption key and nonce
    let key = generate_key();
    let nonce = generate_nonce();

    // Encrypt content and filename
    let encrypted_filename = encrypt(name.trim(), &nonce, &key)?;

    // Trim line ending and join them with a newline
    let content = content
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<&str>>()
        .join("\n");

    let encrypted_content = encrypt(&content, &nonce, &key)?;

    // Create encrypted paste object
    let encrypted_paste = EncryptedPaste {
        filename: encrypted_filename,
        content: encrypted_content,
        expire_after: cli.expire_after,
    };

    // Send request to API
    let api_route = format!("{}/api/paste/new", &cli.url);

    let resp = ureq::post(&api_route).send_json(&encrypted_paste)?;

    // Get paste ID from response
    let paste_response: PasteCreateResponse = resp.into_json()?;

    // Generate key in base64
    let key_base64 = general_purpose::URL_SAFE_NO_PAD.encode(key);

    // Print the URL
    println!("{}/paste/{}#{}", &cli.url, paste_response.id, key_base64);

    Ok(())
}
