use crate::components::safe_html::SafeHtml;
use aes_gcm::aead::consts::U12;
use aes_gcm::aead::{Aead, Nonce};
use aes_gcm::aes::Aes256;
use aes_gcm::{Aes256Gcm, AesGcm, KeyInit};
use base64::{engine::general_purpose, Engine as _};
use gloo_net::http::Request;
use serde::Deserialize;
use std::ffi::OsStr;
use std::path::Path as StdPath;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

const THEME: &str = "base16-eighties.dark";

fn decrypt_paste(paste: &PasteResponse, key_base64: &str) -> Result<PasteResponse, String> {
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

    Ok(PasteResponse { filename, content })
}

fn decrypt_data(cipher: &Aes256Gcm, data: &str) -> Result<String, String> {
    let decoded = general_purpose::URL_SAFE_NO_PAD
        .decode(data)
        .map_err(|e| format!("Failed to decode data: {:?}", e))?;

    if decoded.len() < 12 {
        return Err("Invalid data length".to_string());
    }

    let nonce = Nonce::<AesGcm<Aes256, U12>>::from_slice(&decoded[0..12]);
    tracing::debug!("Nonce: {:?}", nonce);
    let ciphertext = &decoded[12..];
    tracing::debug!("Ciphertext: {:?}", ciphertext);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption error: {:?}", e))?;

    String::from_utf8(plaintext).map_err(|e| format!("UTF-8 decoding error: {:?}", e))
}

#[derive(Deserialize, Debug)]
pub struct PasteResponse {
    filename: String,
    content: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

pub struct GetPaste {
    filename: String,
    content: String,
    loaded: bool,
    error: Option<String>,
}

pub enum Msg {
    PasteLoaded(PasteResponse),
    DecryptionError(String),
}

impl Component for GetPaste {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().id.clone();

        // Get the encryption key from the URL
        let window = window().unwrap();
        let search_params = window.location().search().unwrap();
        let url_params = web_sys::UrlSearchParams::new_with_str(&search_params).unwrap();
        let key_base64 = url_params.get("key").unwrap_or_default();

        let link = ctx.link().clone();

        spawn_local(async move {
            let api_url: &'static str = env!("TOOTHPASTE_API_URL");
            let api_route = format!("{}/paste/{}", api_url, id);
            let resp = Request::get(&api_route).send().await.unwrap();

            if resp.ok() {
                let paste_response: PasteResponse = resp.json().await.unwrap();

                // Decrypt the paste content
                match decrypt_paste(&paste_response, &key_base64) {
                    Ok(decrypted_paste) => link.send_message(Msg::PasteLoaded(decrypted_paste)),
                    Err(e) => link.send_message(Msg::DecryptionError(e)),
                }
            }
        });

        Self {
            filename: String::new(),
            content: String::new(),
            loaded: false,
            error: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PasteLoaded(paste) => {
                self.filename = paste.filename.clone();
                self.content = paste.content.clone();

                self.loaded = true;
                true
            }
            Msg::DecryptionError(error) => {
                self.error = Some(error);
                self.loaded = true;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if !self.loaded {
            html! {
                <section class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                    <h1 class="title">{ "Loading..." }</h1>
                </section>
            }
        } else {
            if let Some(error) = &self.error {
                html! {
                    <section class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                        <h1 class="title">{ "Error" }</h1>
                        <div>{ error }</div>
                    </section>
                }
            } else {
                html! {
                    <section class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                        <h1 class="title">{ &self.filename }</h1>
                        <SafeHtml html={self.highlight_content(&self.filename, &self.content)} />

                    </section>
                }
            }
        }
    }
}

impl GetPaste {
    fn highlight_content(&self, filename: &str, content: &str) -> String {
        let extension = StdPath::new(filename)
            .extension()
            .unwrap_or_else(|| OsStr::new("txt"))
            .to_str()
            .unwrap();

        let ss = SyntaxSet::load_defaults_newlines();
        let syntax = match ss.find_syntax_by_extension(extension) {
            Some(syntax) => syntax,
            None => ss.find_syntax_plain_text(),
        };
        let ts = ThemeSet::load_defaults();

        highlighted_html_for_string(content, &ss, syntax, &ts.themes[THEME])
            .unwrap_or_else(|_| content.to_string())
    }
}
