use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Form,
};
use chrono::{Duration, NaiveDateTime, Utc};
use entity::paste;
use nanoid::nanoid;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::Deserialize;

use crate::SharedState;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Payload {
    filename: String,
    content: String,
    expire_after: i64,
    private: Option<bool>,
}

pub async fn new_paste(
    State(state): State<SharedState>,
    Form(payload): Form<Payload>,
) -> impl IntoResponse {
    let now: NaiveDateTime = Utc::now().naive_utc();

    let private: bool = payload.private.unwrap_or(false);

    let mut new_paste = paste::ActiveModel {
        id: ActiveValue::Set(nanoid!(10)),
        filename: ActiveValue::Set(payload.filename),
        content: ActiveValue::Set(payload.content),
        create_time: ActiveValue::Set(now),
        private: ActiveValue::Set(private),
        ..Default::default()
    };

    // If expire after is present, we create the expire time
    let expire_after = payload.expire_after;

    if expire_after > 0 {
        let expire_time = now + Duration::seconds(expire_after);
        new_paste.expire_time = ActiveValue::Set(Some(expire_time));
    }

    let paste: paste::Model = new_paste
        .insert(&state.db)
        .await
        .expect("Could not insert paste");

    let location = format!("/paste/{}", paste.id);
    Redirect::to(location.as_ref())
}
