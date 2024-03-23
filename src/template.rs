use axum::http::StatusCode;
use tera::{Context, Tera};

pub fn render_or_internal_error(template_name: &str, ctx: &Context, tera: &Tera) -> String {
    tera.render(template_name, ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))
        .unwrap()
}
