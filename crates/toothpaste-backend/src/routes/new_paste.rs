use axum::http::StatusCode;
use axum::{extract::State, Json};
use chrono::{Duration, NaiveDateTime, Utc};
use entity::paste;
use nanoid::nanoid;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};

use crate::SharedState;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Payload {
    filename: String,
    content: String,
    expire_after: i64,
}

#[derive(Serialize)]
pub struct PasteResponse {
    id: String,
}

pub async fn new_paste(
    State(state): State<SharedState>,
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<PasteResponse>), StatusCode> {
    let now: NaiveDateTime = Utc::now().naive_utc();

    let mut new_paste = paste::ActiveModel {
        id: ActiveValue::Set(nanoid!(10)),
        filename: ActiveValue::Set(payload.filename),
        content: ActiveValue::Set(payload.content),
        create_time: ActiveValue::Set(now),
        private: ActiveValue::Set(true),
        ..Default::default()
    };

    // If expire after is present, we create the expiry time
    let expire_after = payload.expire_after;

    if expire_after > 0 {
        let expire_time = now + Duration::try_seconds(expire_after).unwrap();
        new_paste.expire_time = ActiveValue::Set(Some(expire_time));
        new_paste.expire_after = ActiveValue::Set(Some(expire_after));
    }

    let paste: paste::Model = new_paste
        .insert(&state.db)
        .await
        .expect("Could not insert paste");

    let response = PasteResponse { id: paste.id };
    Ok((StatusCode::CREATED, Json(response)))
}
