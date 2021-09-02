use crate::paste::Paste;
use crate::state::State;
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use rbatis::crud::CRUD;
use tide::{Redirect, Request};

pub async fn new(mut req: Request<State>) -> tide::Result {
    let mut paste: Paste = req.body_form().await?;
    paste.create_time = Some(NaiveDateTime::now());

    let state = req.state();
    let pool = state.pool.lock().await;

    let result = pool.save(&paste, &[]).await;
    let id = result.unwrap().last_insert_id.unwrap();

    let location = format!("/paste/{}", id);
    Ok(Redirect::new(location).into())
}
