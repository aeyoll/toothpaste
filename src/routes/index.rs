use crate::{paste::Paste, state::State};
use rbatis::crud::CRUD;
use tide::Request;

pub async fn index(req: Request<State>) -> tide::Result<String> {
    let state = req.state();
    let pool = state.pool.lock().await;
    let v = pool.fetch_list::<Paste>().await;
    Ok(serde_json::json!(v).to_string())
}
