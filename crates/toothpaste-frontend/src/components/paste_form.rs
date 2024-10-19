use gloo_net::http::Request;
use indexmap::indexmap;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;
use yew::{function_component, html, use_reducer, Html, Properties};

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
}

impl Default for PasteState {
    fn default() -> Self {
        Self {
            filename: String::new(),
            content: String::new(),
            expire_after: "86400".to_string(),
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
                content: self.content.clone(),
                expire_after: self.expire_after.clone(),
            }),
            PasteAction::SetContent(content) => Rc::new(PasteState {
                content,
                filename: self.filename.clone(),
                expire_after: self.expire_after.clone(),
            }),
            PasteAction::SetExpireAfter(expire_after) => Rc::new(PasteState {
                expire_after,
                filename: self.filename.clone(),
                content: self.content.clone(),
            }),
            PasteAction::Submit => Rc::new(PasteState {
                expire_after: self.expire_after.clone(),
                filename: self.filename.clone(),
                content: self.content.clone(),
            }),
        }
    }
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
                    wasm_bindgen_futures::spawn_local(async move {
                        let resp = Request::post("/path")
                            .json(&*paste_data)
                            .unwrap()
                            .send()
                            .await
                            .unwrap();
                        // Handle the response as needed
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
