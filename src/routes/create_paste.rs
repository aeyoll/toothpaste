use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use tera::Tera;

use crate::SharedState;

pub async fn create_paste(
    Extension(tera): Extension<Tera>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("private", &state.private);

    let body = tera
        .render("create.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))
        .unwrap();

    (StatusCode::OK, Html(body))
}
