use crate::{state::State, template::IndexTemplate};
use tide::{Request, Response};

pub async fn index(_req: Request<State>) -> tide::Result<Response> {
    let response = IndexTemplate {}.into();
    Ok(response)
}
