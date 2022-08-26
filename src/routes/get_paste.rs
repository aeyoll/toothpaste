use std::{ffi::OsStr, path::Path as StdPath};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use entity::paste;
use paste::Entity as Paste;
use sea_orm::entity::prelude::*;
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use tera::Tera;

use crate::template::render_or_internal_error;
use crate::SharedState;

const THEME: &str = "base16-eighties.dark";

pub async fn get_paste(
    Extension(tera): Extension<Tera>,
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let db = &state.db;
    let mut cache = state.cache.lock().await;
    let cache_key = id.to_string();

    let paste: Option<paste::Model> = Paste::find_by_id(id).one(db).await.unwrap();

    if paste.is_some() {
        let paste = paste.unwrap();
        let html_content;

        if let Some(response) = cache.get(&cache_key) {
            html_content = response.clone();
        } else {
            let filename = &paste.filename;
            let extension = StdPath::new(filename)
                .extension()
                .unwrap_or_else(|| OsStr::new("txt"))
                .to_str()
                .unwrap();

            let s = &paste.content;
            let ss = SyntaxSet::load_defaults_newlines();
            let syntax = match ss.find_syntax_by_extension(extension) {
                Some(syntax) => syntax,
                None => ss.find_syntax_plain_text(),
            };
            let ts = ThemeSet::load_defaults();

            html_content = highlighted_html_for_string(s, &ss, syntax, &ts.themes[THEME]);
            let _ = cache.put_with_weight(cache_key.to_string(), html_content.clone());
        }

        let mut ctx = tera::Context::new();
        ctx.insert("id", &paste.id);
        ctx.insert("filename", &paste.filename);
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
