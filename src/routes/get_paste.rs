use aes_gcm_siv::Nonce;
use clru::CLruCache;
use futures::lock::MutexGuard;
use std::{ffi::OsStr, path::Path as StdPath};

use crate::cryptography::{Cryptography, Key};
use axum::extract::Query;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use entity::paste;
use paste::Entity as Paste;
use sea_orm::entity::prelude::*;
use serde::Deserialize;
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use tera::Tera;

use crate::template::render_or_internal_error;
use crate::SharedState;

const THEME: &str = "base16-eighties.dark";

#[derive(Deserialize, Default)]
pub struct CryptographyQuery {
    key: String,
    nonce: String,
}

fn initialize_cryptography(query: Query<CryptographyQuery>) -> Cryptography {
    let base64_key = &query.key;
    let base64_nonce = &query.nonce;
    let b_key = URL_SAFE.decode(base64_key).unwrap();
    let b_nonce = URL_SAFE.decode(base64_nonce).unwrap();
    let key: Key = b_key.as_slice().try_into().unwrap();
    let nonce = *Nonce::from_slice(b_nonce.as_slice());

    Cryptography::init(key, nonce)
}

fn decode(content: &String, cryptography: &Cryptography) -> String {
    let encoded_string = URL_SAFE.decode(content).unwrap();

    String::from_utf8(cryptography.decrypt(encoded_string)).unwrap()
}

fn get_filename(paste: &paste::Model, cryptography: &Cryptography) -> String {
    decode(&paste.filename, cryptography)
}

fn get_content(paste: &paste::Model, cryptography: &Cryptography) -> String {
    decode(&paste.content, cryptography)
}

fn get_html_content(
    cache: &mut MutexGuard<
        '_,
        CLruCache<
            String,
            String,
            std::hash::BuildHasherDefault<fnv::FnvHasher>,
            crate::cache::StringScale,
        >,
    >,
    cache_key: &str,
    s: String,
    filename: &str,
) -> String {
    if let Some(response) = cache.get(cache_key) {
        response.clone()
    } else {
        // Get the extension from the filename
        let extension = StdPath::new(&filename)
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

        let html_content = highlighted_html_for_string(&s, &ss, syntax, &ts.themes[THEME]).unwrap();
        let _ = cache.put_with_weight(cache_key.to_string(), html_content.clone());

        html_content
    }
}

pub async fn get_paste(
    Extension(tera): Extension<Tera>,
    Path(id): Path<String>,
    State(state): State<SharedState>,
    query: Option<Query<CryptographyQuery>>,
) -> impl IntoResponse {
    let db = &state.db;
    let paste: Option<paste::Model> = Paste::find_by_id(id).one(db).await.unwrap();

    match paste {
        Some(paste) => {
            let filename;
            let content;

            if paste.private {
                let Query(query) = query.unwrap_or_default();
                let cryptography = initialize_cryptography(axum::extract::Query(query));

                filename = get_filename(&paste, &cryptography);
                content = get_content(&paste, &cryptography);
            } else {
                filename = paste.filename;
                content = paste.content;
            }

            let mut cache: MutexGuard<
                '_,
                CLruCache<
                    String,
                    String,
                    std::hash::BuildHasherDefault<fnv::FnvHasher>,
                    crate::cache::StringScale,
                >,
            > = state.cache.lock().await;
            let html_content = get_html_content(&mut cache, &paste.id, content, &filename);

            let mut ctx = tera::Context::new();
            ctx.insert("id", &paste.id);
            ctx.insert("filename", &filename);
            ctx.insert("content", &html_content);

            let body = render_or_internal_error("get_paste.html", &ctx, &tera);
            (StatusCode::OK, Html(body))
        }
        None => {
            let mut ctx = tera::Context::new();
            ctx.insert("message", "Paste not found");

            let body = render_or_internal_error("404.html", &ctx, &tera);
            (StatusCode::NOT_FOUND, Html(body))
        }
    }
}
