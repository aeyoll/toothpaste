use crate::state::State;
use tide::Request;

pub async fn cleanup(_req: Request<State>) -> tide::Result {
    Ok(format!("Cleanup finished").into())
}
