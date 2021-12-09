use crate::{state::State, templates::index::IndexTemplate};
use rbatis::crud::CRUD;
use tide::{Request, Response};

pub async fn index(req: Request<State>) -> tide::Result<Response> {
    let state = req.state();
    let pool = state.pool.lock().await;

    let wrapper = pool
        .new_wrapper()
        .eq("private", false)
        .order_by(false, &["id"]);
    let pastes = match pool.fetch_list_by_wrapper(&wrapper).await {
        Ok(pastes) => pastes,
        Err(_err) => vec![],
    };

    let response = IndexTemplate { pastes }.into();

    Ok(response)
}
