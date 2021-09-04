use crate::{state::State, templates::create::CreateTemplate};
use tide::{Request, Response};

pub async fn create(_req: Request<State>) -> tide::Result<Response> {
    let response = CreateTemplate {}.into();
    Ok(response)
}
