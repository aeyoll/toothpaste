use crate::SharedState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use entity::paste;
use paste::Entity as Paste;
use sea_orm::entity::prelude::*;

pub async fn get_paste(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<(StatusCode, Json<paste::Model>), StatusCode> {
    let db = &state.db;

    let paste: Option<paste::Model> = Paste::find_by_id(id).one(db).await.unwrap();

    if paste.is_some() {
        let paste = paste.unwrap();
        Ok((StatusCode::OK, Json(paste)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
