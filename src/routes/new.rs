use crate::paste::Paste;
use crate::state::State;
use chrono::{Duration, NaiveDateTime};
use rbatis::core::value::DateTimeNow;
use rbatis::crud::CRUD;
use tide::{Redirect, Request};

pub async fn new(mut req: Request<State>) -> tide::Result {
    let mut paste: Paste = req.body_form().await?;
    paste.create_time = Some(NaiveDateTime::now());

    // If expire after is present, we create the expire time
    let expire_after = paste.expire_after.unwrap_or(0);
    if expire_after > 0 {
        let create_time = paste.create_time.unwrap();
        paste.expire_time = Some(create_time + Duration::seconds(expire_after.into()));
    }

    let state = req.state();
    let pool = state.pool.lock().await;

    let result = pool.save(&paste, &[]).await;
    let id = result.unwrap().last_insert_id.unwrap();

    let location = format!("/paste/{}", id);
    Ok(Redirect::new(location).into())
}
