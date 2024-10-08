use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use tera::Tera;

use crate::template::render_or_internal_error;
use crate::SharedState;

pub async fn create_paste(
    Extension(tera): Extension<Tera>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("private", &state.private);

    let body = render_or_internal_error("create.html", &ctx, &tera);

    (StatusCode::OK, Html(body))
}
