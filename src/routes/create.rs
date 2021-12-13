use crate::{state::State, templates::create::CreateTemplate};
use tide::{Request, Response};

pub async fn create(req: Request<State>) -> tide::Result<Response> {
    let state = req.state();
    let response = CreateTemplate {
        private: state.private,
    }
    .into();
    Ok(response)
}
