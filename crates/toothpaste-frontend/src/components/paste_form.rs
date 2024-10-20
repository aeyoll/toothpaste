use aes_gcm::aead::consts::U12;
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, Nonce, OsRng};
use aes_gcm::aes::Aes256;
use aes_gcm::{AeadCore, Aes256Gcm, AesGcm, KeyInit};
use base64::{engine::general_purpose, Engine as _};
use gloo_net::http::Request;
use indexmap::indexmap;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use web_sys::window;
use yew::prelude::*;
use yew::{function_component, html, use_reducer, Html, Properties};

#[derive(Deserialize)]
pub struct PasteCreateResponse {
    id: String,
}

fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

fn generate_nonce() -> Nonce<AesGcm<Aes256, U12>> {
    Aes256Gcm::generate_nonce(&mut OsRng)
}

// Add this function for encryption
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

#[derive(Properties, PartialEq)]
pub struct Props {}

// Reducer's action
enum PasteAction {
    SetFilename(String),
    SetContent(String),
    SetExpireAfter(String),
    Submit,
}

// Reducer's state
#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PasteState {
    filename: String,
    content: String,
    expire_after: String,
    encryption_key: [u8; 32],
}

impl Default for PasteState {
    fn default() -> Self {
        Self {
            filename: String::new(),
            content: String::new(),
            expire_after: "86400".to_string(),
            encryption_key: generate_key(),
        }
    }
}

impl Reducible for PasteState {
    // Reducer action type
    type Action = PasteAction;

    // Reducer function
    fn reduce(self: Rc<PasteState>, action: Self::Action) -> Rc<PasteState> {
        match action {
            PasteAction::SetFilename(filename) => Rc::new(PasteState {
                filename,
                ..self.as_ref().clone()
            }),
            PasteAction::SetContent(content) => Rc::new(PasteState {
                content,
                ..self.as_ref().clone()
            }),
            PasteAction::SetExpireAfter(expire_after) => Rc::new(PasteState {
                expire_after,
                ..self.as_ref().clone()
            }),
            PasteAction::Submit => Rc::new(self.as_ref().clone()),
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct EncryptedPasteState {
    filename: String,
    content: String,
    expire_after: i64,
}

#[function_component(PasteForm)]
pub fn paste_form(_props: &Props) -> Html {
    let paste_data = use_reducer(PasteState::default);

    // Expire_after key/value hashmap
    let expire_after_values = indexmap! {
        "1800" => "30 minutes",
        "21600" => "6 hours",
        "86400" => "1 day",
        "604800" => "1 week",
        "2592000" => "1 month",
        "0" => "forever",
    };

    let update = {
        let paste_data = paste_data.clone();

        Callback::from(move |action: PasteAction| {
            match action {
                PasteAction::SetFilename(filename) => {
                    paste_data.dispatch(PasteAction::SetFilename(filename));
                }
                PasteAction::SetContent(content) => {
                    paste_data.dispatch(PasteAction::SetContent(content));
                }
                PasteAction::SetExpireAfter(expire_after) => {
                    paste_data.dispatch(PasteAction::SetExpireAfter(expire_after));
                }
                PasteAction::Submit => {
                    let paste_data = paste_data.clone();
                    let nonce = generate_nonce();

                    let encrypted_filename =
                        encrypt(&paste_data.filename, &nonce, &paste_data.encryption_key).unwrap();
                    let encrypted_content =
                        encrypt(&paste_data.content, &nonce, &paste_data.encryption_key).unwrap();

                    let key_base64 =
                        general_purpose::URL_SAFE_NO_PAD.encode(paste_data.encryption_key);

                    let encrypted_paste = EncryptedPasteState {
                        filename: encrypted_filename,
                        content: encrypted_content,
                        expire_after: paste_data.expire_after.parse::<i64>().unwrap(),
                    };

                    wasm_bindgen_futures::spawn_local(async move {
                        let api_url: &'static str = env!("TOOTHPASTE_API_URL");
                        let api_route = format!("{}/paste/new", api_url);
                        let resp = Request::post(api_route.as_str())
                            .json(&encrypted_paste)
                            .unwrap()
                            .send()
                            .await
                            .unwrap();

                        // Get the id from the response
                        let resp: PasteCreateResponse = resp.json().await.unwrap();

                        // Redirect to the paste page
                        let location = format!("/paste/{}#{}", resp.id, key_base64);
                        window()
                            .unwrap()
                            .location()
                            .assign(location.as_str())
                            .unwrap();
                    });
                }
            }
        })
    };

    html! {
        <div>
            <form onsubmit={
                let update = update.clone();

                Callback::from(move |e: SubmitEvent| {
                    e.prevent_default();
                    update.emit(PasteAction::Submit);
                })
            }>
                <div class="mb-3">
                    <label for="filename" class="form-label">{"Filename"}</label>
                    <input
                        type="text"
                        name="filename"
                        id="filename"
                        class="form-control"
                        required=true
                        value={paste_data.filename.clone()}
                        onchange={
                            let update = update.clone();

                            Callback::from(move |e: Event| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                update.emit(PasteAction::SetFilename(value));
                            })
                        }
                    />
                    <div class="form-help">{"Pro-tip: use a file extension to get syntax coloration!"}</div>
                </div>

                <div class="mb-3">
                    <label for="content" class="form-label">{"Content"}</label>
                    <textarea
                        name="content"
                        id="content"
                        class="form-control"
                        required=true
                        rows="20"
                        value={paste_data.content.clone()}
                        onchange={
                            let update = update.clone();

                            Callback::from(move |e: Event| {
                                let value = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value();
                                update.emit(PasteAction::SetContent(value));
                            })
                        }
                    ></textarea>
                </div>

                <div class="mb-3">
                    <label for="expire_after" class="form-label">{"Expire"}</label>
                    <select
                        id="expire_after"
                        name="expire_after"
                        class="form-control"
                        onchange={
                            let update = update.clone();
                            Callback::from(move |e: Event| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                update.emit(PasteAction::SetExpireAfter(value));
                            })
                        }>
                        { for expire_after_values.iter().map(|(key, value)| {
                            let selected = *key == "86400";
                            html! {
                                <option value={key.to_string()} selected={selected}>{value}</option>
                            }
                        }) }
                    </select>
                </div>

                <div class="mb-3">
                    <button type="submit" class="btn btn-primary">{"Create"}</button>
                </div>
            </form>
        </div>
    }
}
