use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::{NaiveDateTime, Utc};
use entity::paste;
use paste::Entity as Paste;
use sea_orm::{entity::prelude::*, DatabaseConnection, DeleteResult};

use crate::SharedState;

pub async fn cleanup(State(state): State<SharedState>) -> impl IntoResponse {
    let db: &DatabaseConnection = &state.db;

    let now: NaiveDateTime = Utc::now().naive_utc();
    let zero: i32 = 0;

    let res: Result<DeleteResult, _> = Paste::delete_many()
        .filter(paste::Column::ExpireAfter.gt(zero))
        .filter(paste::Column::ExpireTime.lt(now))
        .exec(db)
        .await;

    match res {
        Ok(deleted_result) => (
            StatusCode::OK,
            Json(format!("{} paste cleaned up", deleted_result.rows_affected)),
        ),
        Err(..) => (
            StatusCode::BAD_REQUEST,
            Json("Failed to cleanup".to_string()),
        ),
    }
}
