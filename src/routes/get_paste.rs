use std::{ffi::OsStr, path::Path as StdPath};
use aes_gcm_siv::{Nonce};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use axum::extract::Query;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use entity::paste;
use paste::Entity as Paste;
use sea_orm::entity::prelude::*;
use serde::Deserialize;
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use tera::Tera;
use crate::cryptography::{Cryptography, Key};

use crate::template::render_or_internal_error;
use crate::SharedState;

const THEME: &str = "base16-eighties.dark";

#[derive(Deserialize)]
pub struct CryptographyQuery {
    key: String,
    nonce: String,
}

impl Default for CryptographyQuery {
    fn default() -> Self {
        Self {
            key: String::from(""),
            nonce: String::from(""),
        }
    }
}

pub async fn get_paste(
    Extension(tera): Extension<Tera>,
    Path(id): Path<String>,
    State(state): State<SharedState>,
    query: Option<Query<CryptographyQuery>>,
) -> impl IntoResponse {
    let db = &state.db;
    let mut cache = state.cache.lock().await;
    let cache_key = id.to_string();

    let paste: Option<paste::Model> = Paste::find_by_id(id).one(db).await.unwrap();

    if paste.is_some() {
        let paste = paste.unwrap();

        // Fetch base64 key and nonce from query parameters
        let Query(query) = query.unwrap_or_default();
        let base64_key = query.key;
        let base64_nonce = query.nonce;

        // Decode base64 data
        let b_key = URL_SAFE.decode(&base64_key).unwrap();
        let b_nonce = URL_SAFE.decode(&base64_nonce).unwrap();

        let key: Key = b_key.as_slice().try_into().unwrap(); // Convert &[u8] to [u8; 32]
        let nonce = *Nonce::from_slice(b_nonce.as_slice());

        let cryptography = Cryptography::init(key, nonce);

        // Decode filename
        let base64_filename = &paste.filename;
        let encoded_filename= URL_SAFE.decode(base64_filename).unwrap();
        let filename = String::from_utf8(cryptography.decrypt(encoded_filename)).unwrap();

        let html_content;

        if let Some(response) = cache.get(&cache_key) {
            html_content = response.clone();
        } else {
            // Get the extension from the filename
            let extension = StdPath::new(&filename)
                .extension()
                .unwrap_or_else(|| OsStr::new("txt"))
                .to_str()
                .unwrap();

            let base64_content = &paste.content;
            let encoded_content= URL_SAFE.decode(base64_content).unwrap();

            let s = &String::from_utf8(cryptography.decrypt(encoded_content)).unwrap();
            let ss = SyntaxSet::load_defaults_newlines();
            let syntax = match ss.find_syntax_by_extension(extension) {
                Some(syntax) => syntax,
                None => ss.find_syntax_plain_text(),
            };
            let ts = ThemeSet::load_defaults();

            html_content = highlighted_html_for_string(s, &ss, syntax, &ts.themes[THEME]).unwrap();
            let _ = cache.put_with_weight(cache_key.to_string(), html_content.clone());
        }

        let mut ctx = tera::Context::new();
        ctx.insert("id", &paste.id);
        ctx.insert("filename", &filename);
        ctx.insert("content", &html_content);

        let body = render_or_internal_error("get_paste.html", &ctx, &tera);
        (StatusCode::OK, Html(body))
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("message", "Paste not found");

        let body = render_or_internal_error("404.html", &ctx, &tera);
        (StatusCode::NOT_FOUND, Html(body))
    }
}
