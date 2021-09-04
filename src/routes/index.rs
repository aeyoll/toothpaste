use rbatis::crud::CRUD;
use crate::{paste::Paste, state::State, template::IndexTemplate};
use tide::{Request, Response};

pub async fn index(req: Request<State>) -> tide::Result<Response> {
    let state = req.state();
    let pool = state.pool.lock().await;
    let pastes = match pool.fetch_list::<Paste>().await {
        Ok(pastes) => pastes,
        Err(_err) => vec![],
    };

    let response = IndexTemplate {
        pastes
    }.into();

    Ok(response)
}
