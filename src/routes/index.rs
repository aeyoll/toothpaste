use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use entity::paste;
use paste::Entity as Paste;
use sea_orm::{entity::prelude::*, DatabaseConnection};
use tera::Tera;

use crate::template::render_or_internal_error;
use crate::SharedState;

pub async fn index(
    Extension(tera): Extension<Tera>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let db: &DatabaseConnection = &state.db;

    let pastes: Vec<paste::Model> = Paste::find()
        .filter(paste::Column::Private.eq(false))
        .all(db)
        .await
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("pastes", &pastes);

    let body = render_or_internal_error("index.html", &ctx, &tera);

    (StatusCode::OK, Html(body))
}
