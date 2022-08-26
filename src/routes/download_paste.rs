use axum::{
    extract::{Path, State},
    http::{header::CONTENT_DISPOSITION, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use entity::paste;
use paste::Entity as Paste;
use sea_orm::entity::prelude::*;
use tera::Tera;

use crate::SharedState;

pub async fn download_paste(
    Extension(tera): Extension<Tera>,
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let db = &state.db;

    let paste: Option<paste::Model> = Paste::find_by_id(id).one(db).await.unwrap();

    match paste {
        Some(paste) => {
            let content_disposition = format!("attachment; filename=\"{}\"", paste.filename);
            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_DISPOSITION,
                HeaderValue::from_str(&content_disposition).unwrap(),
            );
            let content = paste.content;

            (StatusCode::OK, headers, content).into_response()
        }
        None => {
            let mut ctx = tera::Context::new();
            ctx.insert("message", "Paste not found");

            let body = tera
                .render("404.html", &ctx)
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))
                .unwrap();

            (StatusCode::NOT_FOUND, Html(body)).into_response()
        }
    }
}
