use chrono::{DateTime, Utc};
use gloo_net::http::Request;
use gloo_timers::callback::Timeout;
use humantime::format_duration;
use std::ffi::OsStr;
use std::path::Path as StdPath;
use std::time::Duration;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use toothpaste_encrypt::{decrypt_paste, PasteResponse};
use wasm_bindgen_futures::wasm_bindgen::JsCast;
use wasm_bindgen_futures::{js_sys, spawn_local};
use web_sys::window;
use yew::prelude::*;

const THEME: &str = "base16-eighties.dark";

enum CopyState {
    Idle,
    Copied,
}

const COPY_TO_CLIPBOARD_TEXT: &str = "Copy to clipboard";
const COPIED_TO_CLIPBOARD_TEXT: &str = "Copied to clipboard";

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

pub struct GetPaste {
    filename: String,
    content: String,
    loaded: bool,
    copy_state: CopyState,
    error: Option<String>,
    expire_time: Option<String>,
    remaining_time: Option<String>,
}

impl GetPaste {
    fn format_remaining_time(&self) -> Option<String> {
        let expire_time = self.expire_time.as_ref()?;
        let expire_datetime = DateTime::parse_from_rfc3339(&format!("{}Z", expire_time)).ok()?;
        let now = Utc::now();

        if expire_datetime <= now {
            return Some("Expired".to_string());
        }

        let duration = expire_datetime.signed_duration_since(now);
        let std_duration = Duration::from_secs(duration.num_seconds().max(0) as u64);

        Some(format!("Expires in {}", format_duration(std_duration)))
    }
}

pub enum Msg {
    PasteLoaded(PasteResponse),
    DecryptionError(String),
    StartCopying,
    ResetCopyState,
    DownloadFile,
}

impl Component for GetPaste {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().id.clone();

        // Get the encryption key from the URL
        let window = window().unwrap();
        let hash = window.location().hash().unwrap_or_default();
        let key_base64 = hash.trim_start_matches('#').to_string();

        let link = ctx.link().clone();

        spawn_local(async move {
            let api_url = env!("TOOTHPASTE_API_URL").to_string();
            let api_route = format!("{}/api/paste/{}", api_url, id);
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
            copy_state: CopyState::Idle,
            loaded: false,
            error: None,
            expire_time: None,
            remaining_time: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PasteLoaded(paste) => {
                self.filename = paste.filename.clone();
                self.content = paste.content.clone();
                self.expire_time = paste.expire_time.clone();
                self.remaining_time = self.format_remaining_time();
                self.loaded = true;
                true
            }
            Msg::DecryptionError(error) => {
                self.error = Some(error);
                self.loaded = true;
                true
            }
            Msg::StartCopying => {
                self.copy_state = CopyState::Copied;
                self.copy_to_clipboard();
                let link = ctx.link().clone();
                spawn_local(async move {
                    Timeout::new(5000, move || link.send_message(Msg::ResetCopyState)).forget();
                });
                true
            }
            Msg::ResetCopyState => {
                self.copy_state = CopyState::Idle;
                true
            }
            Msg::DownloadFile => {
                self.download_file();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.loaded {
            html! {
                <section class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                    <h1 class="title">{ "Loading..." }</h1>
                </section>
            }
        } else if let Some(error) = &self.error {
            html! {
                <section class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                    <h1 class="title">{ "Error" }</h1>
                    <div>{ error }</div>
                </section>
            }
        } else {
            let link = ctx.link();
            html! {
                <section class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                    <h1 class="title">{ &self.filename }</h1>
                    {
                        if let Some(remaining) = &self.remaining_time {
                            html! {
                                <div class="mb-4 text-sm text-gray-500">
                                    { remaining }
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <div class="mb-6">
                        <button
                            class="btn btn-primary mr-2"
                            onclick={link.callback(|_| Msg::StartCopying)}
                        >
                            { match self.copy_state {
                                CopyState::Idle => COPY_TO_CLIPBOARD_TEXT,
                                CopyState::Copied => COPIED_TO_CLIPBOARD_TEXT,
                            } }
                        </button>
                        <button
                            class="btn btn-primary"
                            onclick={link.callback(|_| Msg::DownloadFile)}
                        >
                            { "Download File" }
                        </button>
                    </div>
                    {self.highlight_content(&self.filename, &self.content)}
                </section>
            }
        }
    }
}

impl GetPaste {
    fn highlight_content(&self, filename: &str, content: &str) -> Html {
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

        let html = highlighted_html_for_string(content, &ss, syntax, &ts.themes[THEME])
            .unwrap_or_else(|_| content.to_string());

        Html::from_html_unchecked(AttrValue::from(html))
    }

    fn copy_to_clipboard(&self) -> bool {
        let window = window().unwrap();
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let _ = clipboard.write_text(&self.content);

        true
    }

    fn download_file(&self) {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let blob =
            web_sys::Blob::new_with_str_sequence(&js_sys::Array::of1(&self.content.clone().into()))
                .unwrap();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

        let a = document.create_element("a").unwrap();
        a.set_attribute("href", &url).unwrap();
        a.set_attribute("download", &self.filename).unwrap();
        a.set_attribute("style", "display: none").unwrap();

        document.body().unwrap().append_child(&a).unwrap();
        a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
        document.body().unwrap().remove_child(&a).unwrap();

        web_sys::Url::revoke_object_url(&url).unwrap();
    }
}
