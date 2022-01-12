use crate::paste::Paste;
use crate::state::State;
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use rbatis::crud::CRUD;
use tide::{Request, Response};

pub async fn cleanup(req: Request<State>) -> tide::Result<Response> {
    let state = req.state();
    let pool = state.pool.lock().await;

    let now = NaiveDateTime::now();
    let wrapper = pool
        .new_wrapper()
        .gt("expire_after", 0)
        .lt("expire_time", now);
    let _ = pool.remove_by_wrapper::<Paste>(wrapper).await;

    Ok("Cleanup finished".to_string().into())
}
